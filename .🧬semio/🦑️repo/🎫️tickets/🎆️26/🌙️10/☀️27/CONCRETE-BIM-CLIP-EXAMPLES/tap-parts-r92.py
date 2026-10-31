"""🚰 Author inferred hollow tap assemblies through the existing first-party tube interface."""
import math

def tap_parts(api,model,base,joint,outlet,diameter=.025,wall=.002,segments=32):
    """🔧 Build complementary hollow miters, a real valve collar and four source-qualified cross arms."""
    direction=[outlet[i]-joint[i] for i in range(3)];length=math.sqrt(sum(v*v for v in direction));normal=[direction[i]/length+(1. if i==2 else 0.) for i in range(3)];plane={'normal':normal,'point':joint};parts=[]
    for role,points,clip in [('tap_stem',[base,joint],{'clip_end':plane}),('tap_spout',[joint,outlet],{'clip_start':plane})]:
        obj={'guid':api.global_id('TAP-AUTHORING-'+role),'tag':'TAP-AUTHORING-'+role,'class':'IfcPipeSegment','storey':'ground','system':'water','material':'MAT-METAL','geometry':{'points':points,'tube':{'wall_thickness':{'value':wall},'segments':segments,**clip}},'attributes':{'diameter':{'value':diameter}}}
        for p in api._service(model,obj)['parts']:
            body={k:p[k] for k in ('mesh','section','outline','top_outline','z0','z1') if k in p}
            if 'mesh' in body:body={'mesh':body['mesh']}
            elif 'section' in body:body={'section':body['section']}
            parts.append({**body,'material':'MAT-METAL','role':role})
    cx,cy=base[:2];half=.0175;radius=diameter/2;lower=joint[2]-.09;upper=joint[2]-.07
    def emit(out,z0,z1,role):
        """🧱 Emit one nonoverlapping metal valve or handle component."""
        parts.append({'outline':out,'z0':z0,'z1':z1,'material':'MAT-METAL','role':role})
    outer=[];inner=[]
    for i in range(segments):
        a=2*math.pi*i/segments;c,s=math.cos(a),math.sin(a);scale=half/max(abs(c),abs(s));outer.append([cx+scale*c,cy+scale*s]);inner.append([cx+radius*c,cy+radius*s])
    for i in range(segments):
        j=(i+1)%segments;emit([outer[i],outer[j],inner[j],inner[i]],lower,upper,'tap_valve_collar')
    for x0,y0,x1,y1 in [(half,-.0035,.032,.0035),(-.032,-.0035,-half,.0035),(-.0035,half,.0035,.032),(-.0035,-.032,.0035,-half)]:emit([[cx+x0,cy+y0],[cx+x1,cy+y0],[cx+x1,cy+y1],[cx+x0,cy+y1]],lower+.006,upper-.006,'tap_cross_handle')
    return parts
