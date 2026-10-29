"""🚗 Bounded test-only physical packing search; no source geometry mutations."""
import importlib
import json
import math
from pathlib import Path
import time
import sys

Path(__file__).parent.joinpath("🗑️generated").mkdir(exist_ok=True)
Path(__file__).parent.joinpath("🗑️generated/parking-r20-phase.txt").write_text("[DEBUG] before numpy",encoding="utf-8")
import numpy as np
Path(__file__).parent.joinpath("🗑️generated/parking-r20-phase.txt").write_text("[DEBUG] before scipy.optimize",encoding="utf-8")
from scipy.optimize import differential_evolution,minimize
Path(__file__).parent.joinpath("🗑️generated/parking-r20-phase.txt").write_text("[DEBUG] imported scipy.optimize",encoding="utf-8")
from shapely.geometry import Polygon,MultiPoint,Point
from shapely.strtree import STRtree

TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/'🎓️teaching').exists())
if str(ROOT) not in sys.path:sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
INPUT=TICKET/'parking-r20-input.json'
OUT=TICKET/'🗑️generated'


def build_problem():
    """🧱 Capture all actual physical projections that intersect the full vehicle height."""
    cfg=json.loads(INPUT.read_text(encoding='utf-8'))
    api._source.cache_clear()
    source=api.load_model()
    (OUT/"parking-r20-phase.txt").write_text("[DEBUG] building physical geometry",encoding="utf-8")
    geometry=api.building_geometry(source)
    (OUT/"parking-r20-phase.txt").write_text("[DEBUG] physical geometry complete",encoding="utf-8")
    outline=next(s['outline'] for s in geometry['spaces'] if s['tag']=='SP-GARAGE')
    garage=Polygon(outline)
    low=cfg['vehicle']['z0'];high=low+cfg['vehicle']['height']
    records=[]
    for group in ['pilotis','walls','fillings','details','furnishings','railings','beams','foundations','stairs']:
        for obj in geometry.get(group,[]):
            parts=obj.get('parts',obj.get('steps',[]))
            for p in parts:
                if min(p['z1'],high)-max(p['z0'],low)<=1e-9:continue
                shape=MultiPoint(p['outline']+p.get('top_outline',[])).convex_hull
                margin=cfg['clearance']['column_surface'] if group=='pilotis' else cfg['clearance']['physical_surface']
                records.append({'tag':obj['tag'],'kind':'column' if group=='pilotis' else group,'role':p.get('role','solid'),'outline':list(shape.exterior.coords)[:-1],'z0':p['z0'],'z1':p['z1'],'margin':margin})
    reserves=[]
    opening_map={o['tag']:(w,o) for w in geometry['walls'] for o in w['openings']}
    for spec in cfg['pedestrian_reserves']:
        wall,op=opening_map[spec['opening']]
        filling=next(f for f in geometry['fillings'] if f.get('opening')==op['tag'])
        a,b=wall['start'],wall['end'];length=math.dist(a,b);dx,dy=(b[0]-a[0])/length,(b[1]-a[1])/length
        p0=[a[0]+dx*filling['clear_start'],a[1]+dy*filling['clear_start']];p1=[a[0]+dx*filling['clear_end'],a[1]+dy*filling['clear_end']]
        nx,ny=-dy,dx
        centre=[(p0[i]+p1[i])/2 for i in (0,1)]
        if not garage.covers(Point(centre[0]+nx*.1,centre[1]+ny*.1)):nx,ny=-nx,-ny
        ring=[p0,p1,[p1[0]+nx*spec['depth'],p1[1]+ny*spec['depth']],[p0[0]+nx*spec['depth'],p0[1]+ny*spec['depth']]]
        reserves.append({'opening':op['tag'],'polygon':ring,'depth':spec['depth'],'portal_indices':[0,1]})
    (OUT/"parking-r20-phase.txt").write_text("[DEBUG] physical projections complete",encoding="utf-8")
    blocked=[Polygon(r['outline']).buffer(r['margin']+.0005,join_style='mitre') if r['margin'] else Polygon(r['outline']) for r in records]+[Polygon(r['polygon']) for r in reserves]
    snapshot={'scenario':cfg['scenario'],'vehicle':cfg['vehicle'],'garage':outline,'obstacles':records,'pedestrian_reserves':reserves,'garage_opening':cfg['garage_opening'],'source_vehicle':{k:source['car'][k] for k in ['length','width','height']}}
    OUT.mkdir(exist_ok=True)
    (OUT/'parking-r20-physical.json').write_text(json.dumps(snapshot,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    return cfg,source,geometry,garage,blocked,snapshot


def rectangle(pose,vehicle):
    """📐 Construct unchanged vehicle corners with a NumPy rigid transform."""
    x,y,theta=pose
    angle=math.radians(theta);c,s=math.cos(angle),math.sin(angle)
    points=np.array([[-1,-1],[1,-1],[1,1],[-1,1]])*np.array([vehicle['length'],vehicle['width']])/2
    return Polygon(points@np.array([[c,s],[-s,c]])+np.array([x,y]))


class StopSearch(Exception):
    pass


class ParkingSearch:
    """⏱️ Limit each search chunk and retain the best candidate for deterministic continuation."""
    def __init__(self):
        self.cfg,self.source,self.geometry,self.garage,self.blocked,self.snapshot=build_problem()
        self.usable=self.garage.buffer(-self.cfg['clearance']['boundary']-.0005,join_style='mitre')
        self.tree=STRtree(self.blocked)
        self.best=np.array([13.5,9.8,140,13.4,12.2,140,13.44,15.3,160],dtype=float)
        self.score=float('inf');self.evaluations=0;self.chunks=0;self.population=None;self.history=[]
        self.bounds=[(11,15.9),(5,17.9),(0,180)]*3
        self.deadline=float('inf')

    def objective(self,vector):
        if time.monotonic()>self.deadline or json.loads(INPUT.read_text(encoding='utf-8')).get('cancel_requested'):raise StopSearch()
        self.evaluations+=1
        cars=[rectangle(vector[i:i+3],self.cfg['vehicle']) for i in (0,3,6)]
        value=sum(c.difference(self.usable).area+sum(c.intersection(self.blocked[int(i)]).area for i in self.tree.query(c)) for c in cars)
        for i,c in enumerate(cars):
            for other in cars[:i]:value+=c.intersection(other).area+max(0,self.cfg['clearance']['car_pair']+.0005-c.distance(other))**2
        if value<self.score:self.score=float(value);self.best=np.array(vector)
        return value

    def run_chunk(self):
        """🧭 Run one bounded local or differential-evolution batch and persist progress."""
        if self.chunks>=self.cfg['search']['max_chunks']:raise ValueError('Parking chunk budget exhausted')
        self.chunks+=1;start=time.monotonic();self.deadline=start+self.cfg['search']['max_seconds_per_chunk']
        try:
            if self.chunks==1:
                result=minimize(self.objective,self.best,method='Nelder-Mead',bounds=self.bounds,options={'maxfev':self.cfg['search']['max_local_evaluations'],'xatol':1e-7,'fatol':1e-13,'adaptive':True})
            else:
                rng=np.random.default_rng(self.cfg['search']['seed']+self.chunks)
                count=self.cfg['search']['population_size']
                if self.population is None:
                    initial=np.array([[rng.uniform(lo,hi) for lo,hi in self.bounds] for _ in range(count)])
                    for i in range(count//2):initial[i]=self.best+rng.normal(0,[.3,.4,8]*3)
                    initial=np.maximum(np.minimum(initial,np.array(self.bounds)[:,1]),np.array(self.bounds)[:,0]);initial[0]=self.best
                else:initial=self.population
                result=differential_evolution(self.objective,self.bounds,init=initial,seed=self.cfg['search']['seed']+self.chunks,maxiter=1000,polish=False,updating='immediate',tol=1e-10,callback=lambda x,convergence:self.score<1e-12)
                self.population=result.population
        except StopSearch:pass
        self.deadline=float('inf')
        entry={'chunk':self.chunks,'elapsed_seconds':time.monotonic()-start,'evaluations':self.evaluations,'score':self.score,'poses':self.best.reshape(3,3).tolist()}
        self.history.append(entry)
        (OUT/'parking-r20-search.json').write_text(json.dumps({'input':self.cfg,'history':self.history},indent=2)+'\n',encoding='utf-8')
        print('[DEBUG] parking',json.dumps(entry))
        return entry


def sat_refinement(search,max_seconds=20):
    """📏 Refine a fleet with signed separating-axis constraints and a bounded common margin."""
    start=time.monotonic();vehicle=search.cfg['vehicle'];margin=search.cfg['clearance']
    hull=np.array(search.garage.convex_hull.exterior.coords)[:-1]
    if not search.garage.convex_hull.exterior.is_ccw:hull=hull[::-1]
    edge=np.roll(hull,-1,axis=0)-hull;normal=np.column_stack([-edge[:,1],edge[:,0]]);normal/=np.linalg.norm(normal,axis=1)[:,None]
    def obstacle(ring,gap):
        points=np.array(ring,dtype=float);edge=np.roll(points,-1,axis=0)-points;axes=np.column_stack([-edge[:,1],edge[:,0]]);axes/=np.linalg.norm(axes,axis=1)[:,None]
        radii=np.linalg.norm(points-points.mean(axis=0),axis=1)
        circle=(points.mean(axis=0),radii.max()) if len(points)>8 and np.ptp(radii)<1e-5 else None
        return points,axes,points@axes.T,gap,circle
    obstacles=[obstacle(r['outline'],r['margin']+.0005) for r in search.snapshot['obstacles'] if r['kind']=='column']+[obstacle(r['polygon'],.0005) for r in search.snapshot['pedestrian_reserves']]
    obstacles.append(obstacle([[-100,-100],[12,-100],[12,10],[-100,10]],margin['boundary']+.0005))
    def car(pose):
        x,y,theta=pose;angle=math.radians(theta);c,s=math.cos(angle),math.sin(angle);axes=np.array([[c,s],[-s,c]])
        points=np.array([[-1,-1],[1,-1],[1,1],[-1,1]])*np.array([vehicle['length'],vehicle['width']])/2
        return points@axes+np.array([x,y]),axes
    def separation(points,axes,other):
        op,oa,projected,gap=other[:4]
        if len(other)>4 and other[4] is not None:
            centre,radius=other[4];q=np.abs((centre-points.mean(axis=0))@axes.T)-np.array([vehicle['length']/2,vehicle['width']/2])
            return np.linalg.norm(np.maximum(q,0))+min(max(q),0)-radius-gap
        pa=points@oa.T;pb=op@axes.T;pc=points@axes.T
        return max(np.max(projected.min(axis=0)-pa.max(axis=0)),np.max(pa.min(axis=0)-projected.max(axis=0)),np.max(pb.min(axis=0)-pc.max(axis=0)),np.max(pc.min(axis=0)-pb.max(axis=0)))-gap
    def constraints(vector):
        cars=[car(vector[i:i+3]) for i in (0,3,6)];values=[]
        for points,axes in cars:
            values.extend(((points[:,None,:]-hull[None,:,:])*normal[None,:,:]).sum(axis=2).ravel()-margin['boundary']-.0005)
            values.extend(separation(points,axes,o) for o in obstacles)
        for i,(points,axes) in enumerate(cars):
            for op,oa in cars[:i]:values.append(separation(points,axes,(op,oa,op@oa.T,margin['car_pair']+.0005)))
        return np.array(values)-vector[-1]
    def callback(vector):
        search.sat_vector=np.array(vector)
        search.sat_minimum=float(constraints(vector).min())
        if time.monotonic()-start>max_seconds or json.loads(INPUT.read_text(encoding='utf-8')).get('cancel_requested'):raise StopSearch()
    initial=getattr(search,'sat_vector',np.r_[search.best,-.1]);bounds=search.bounds+[(-.2,.05)]
    try:
        result=minimize(lambda x:-x[-1],initial,method='SLSQP',bounds=bounds,constraints=[{'type':'ineq','fun':constraints}],callback=callback,options={'maxiter':400,'ftol':1e-11})
        vector=result.x;success=bool(result.success);message=str(result.message)
    except StopSearch:
        vector=search.sat_vector;success=False;message='bounded time limit; state retained'
    candidate=vector[:-1];previous=search.score;value=search.objective(candidate)
    report={'method':'signed-axis-SLSQP','elapsed_seconds':time.monotonic()-start,'success':success,'message':message,'common_margin':float(vector[-1]),'minimum_constraint':float(constraints(vector).min()),'physical_penalty':float(value),'poses':candidate.reshape(3,3).tolist()}
    search.sat_reports=getattr(search,'sat_reports',[])+[report]
    (OUT/'parking-r20-sat.json').write_text(json.dumps(search.sat_reports,indent=2)+'\n',encoding='utf-8');print('[DEBUG] parking',json.dumps(report));return report


if __name__=='__main__':
    search=ParkingSearch()
    search.run_chunk()
    search.run_chunk()
    sat_refinement(search,20)
    for angles in [[138,150,160],[150,138,150],[130,150,165],[145,130,165]]:
        vector=search.best.copy();vector[[2,5,8]]=angles;search.sat_vector=np.r_[vector,-.1]
        sat_refinement(search,5)
    print('[DEBUG] final bounded parking penalty',search.score)
