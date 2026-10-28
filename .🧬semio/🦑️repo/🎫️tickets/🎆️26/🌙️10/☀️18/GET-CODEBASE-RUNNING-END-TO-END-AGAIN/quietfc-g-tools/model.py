"""quietfc-g Python model of the host-capacity estimator + quiet forecast (independent port of 📐️estimates + 🌦️forecast)."""
import json,math
POL=dict(halfLife=259200000,quantile=9000,margin=2500,minimumSamples=3,maximumSamples=16,minRatio=3000,maxRatio=20000,
 defaultRamp=300000,maxRamp=1800000,rampMs=60000,quick=120000,quietRef=2048,quietShare=8000,quietMin=2,quietMax=4096)
U=1_000_000
def rnd(x): return math.floor(x+0.5)
def weights(pol,samples,now): return [rnd(U*2**(-max(0,now-s['atMs'])/pol['halfLife'])) for s in samples]
def quantile(samples,w,field,bp):
    val=lambda s: s.get('peakAtMilliseconds',s['activeMilliseconds']) if field=='peakAtMilliseconds' else s[field]
    rows=sorted(zip(samples,w),key=lambda r:(val(r[0]),r[0]['atMs']))
    total=sum(r[1] for r in rows); target=math.ceil(total*bp/10000); c=0
    for s,wt in rows:
        c+=wt
        if c>=target: return val(s)
    return val(rows[-1][0])
def estimate(pol,store,lineage,now):
    ratio=10000; ramp=pol['defaultRamp']; learned=0; quick=False
    for key in lineage:
        samples=store.get(key,[]); w=weights(pol,samples,now); total=sum(w)
        if not samples or total==0:
            ratio=min(10000,ratio); ramp=min(pol['defaultRamp'],ramp); quick=False; continue
        eff=total/U; peak=rnd(quantile(samples,w,'ratioBasisPoints',pol['quantile'])*(10000+pol['margin'])/10000); trust=min(1,eff/pol['minimumSamples'])
        late=rnd(quantile(samples,w,'peakAtMilliseconds',pol['quantile'])*(10000+pol['margin'])/10000)
        ratio=rnd(trust*peak+(1-trust)*min(10000,ratio)); ramp=rnd(trust*late+(1-trust)*min(pol['defaultRamp'],ramp)); learned=rnd(eff)
        quick=eff>=pol['minimumSamples'] and pol['quick']>0 and quantile(samples,w,'activeMilliseconds',5000)<=pol['quick']
    return dict(ratio=max(pol['minRatio'],min(pol['maxRatio'],ratio)),ramp=max(pol['rampMs'],min(pol['maxRamp'],ramp)),samples=learned,quick=quick)
def record(pol,store,keys,atMs,peakMb,defaultMb,active,peakAt):
    store={k:list(v) for k,v in store.items()}
    ratio=max(1,rnd(peakMb*10000/defaultMb))
    for k in keys:
        rows=store.setdefault(k,[]); rows.append(dict(atMs=atMs,ratioBasisPoints=ratio,activeMilliseconds=active,peakAtMilliseconds=min(peakAt,active)))
        rows.sort(key=lambda r:(-r['atMs'],r['ratioBasisPoints'],r['activeMilliseconds'],r.get('peakAtMilliseconds',r['activeMilliseconds'])))
        del rows[pol['maximumSamples']:]
    return store
def forecast(pol,store,leaf,now):
    q=store.get(leaf+'~quiet',[]); b=store.get(leaf+'~busy',[])
    qw=sum(weights(pol,q,now)); bw=sum(weights(pol,b,now)); tot=qw+bw
    share=0 if tot==0 else math.floor(qw*10000/tot+0.5)
    newest=lambda rows:max([r['atMs'] for r in rows],default=-1)
    est=estimate(pol,store,[leaf+'~quiet'],now); mb=math.floor(est['ratio']*pol['quietRef']/10000+0.5)
    return dict(quiet=qw>=pol['quietMin']*U and share>=pol['quietShare'] and newest(q)>newest(b) and mb<=pol['quietMax'],megabytes=mb,share=share,samples=est['samples'])
