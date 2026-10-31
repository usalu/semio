from lib import *
CLO=0.155
VAP=lambda R:round(R/(0.38*0.0165),2)   # Re = I / (i_m L_R), i_m 0.38, L_R 16.5 K/kPa -> m2 Pa / W
REFS={
 "gagge-1971":"Gagge, A. P., Stolwijk, J. A. J., Nishi, Y. (1971). An effective temperature scale based on a simple model of human physiological regulatory response. ASHRAE Transactions 77(1), 247-262 (two-node model; skin blood flow limits 6.3 and 90 L/(m2 h) after the common implementation).",
 "gupta-2010":"Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39 (breathing flow close to a sinusoid, mouth and nose openings, jet directions).",
 "hoeppe-1981":"Hoeppe, P. (1981). Temperatures of expired air under varying climatic conditions. International Journal of Biometeorology 25, 127-132 (expired air 29 to 34 degrees Celsius depending on ambient temperature).",
 "astrand-rodahl-2003":"Astrand, P.-O., Rodahl, K., Dahl, H. A., Stromme, S. B. (2003). Textbook of Work Physiology, 4th ed., Human Kinetics (ventilation about 25 to 30 litres per litre oxygen consumed, tidal volume rising to 1.5 to 2.5 litres in running).",
 "bramble-carrier-1983":"Bramble, D. M., Carrier, D. R. (1983). Running and breathing in mammals. Science 219, 251-256 (locomotor-respiratory coupling, whole-number ratios of strides to breaths).",
 "winkworth-1995":"Winkworth, A. L., Davis, P. J., Ellis, E., Adams, R. D. (1995). Breathing patterns during spontaneous speech. Journal of Speech and Hearing Research 38, 124-144 (short inspirations, long expirations over phrases of several seconds).",
 "mifflin-1990":"Mifflin, M. D. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51, 241-247 (basal metabolism 10 m + 6.25 h - 5 age + 5, kcal/day).",
 "incropera-2011":"Incropera, F. P., DeWitt, D. P. et al. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley (conduction, convection correlations, material properties).",
}
def refs(*ids): return [{"id":i,"citation":REFS[i]} for i in ids]
def dubois(m,h_cm): return 0.007184*m**0.425*h_cm**0.725
def mifflin(m,h_cm,age=30): return (10*m+6.25*h_cm-5*age+5)*4184/86400
def layer(id,en,de,kind,material,res=None,clo=None,thick=None,cap=None,vap=True,src="authored-estimate",ref=None):
    R=res if res is not None else round(clo*CLO,4)
    o={"id":id,"title":T(en,de),"kind":kind,"material":material}
    if thick is not None: o["thicknessM"]=thick
    o["resistanceM2KW"]=R
    if kind!="air-gap" and vap is not False: o["vapourResistanceM2PaW"]=VAP(R) if vap is True else vap
    if cap is not None: o["heatCapacityJM2K"]=cap
    o["source"]=src
    if ref: o["reference"]=ref
    return o
def airgap(id,en,de,R,material,thick=0.004,ref="iso-9920-2007"):
    return layer(id,en,de,"air-gap",material,res=R,thick=thick,vap=False,ref=ref,src="authored-estimate")
def region(id,en,de,kind,material,area,Lc,geom,svf,layers,cap,seg=None,share=None,skin=None,model="body",src="authored-estimate",ref=None,**extra):
    o={"id":id,"title":T(en,de),"kind":kind,"model":model,"material":material,"areaM2":area,"characteristicLengthM":Lc,"geometry":geom,"skyViewFactor":svf,"layers":layers,"heatCapacityJM2K":cap}
    if seg: o["body"]={"segment":seg,"metabolicShare":share,"skinFraction":skin}
    o.update(extra); o["source"]=src
    if ref: o["reference"]=ref
    return o
def breath_pattern(id,en,de,rate,vt,i,pi,e,pe,route,ref,src="authored-estimate"):
    assert abs(i+pi+e+pe-1)<1e-9,id
    return {"id":id,"title":T(en,de),"rateBpm":rate,"tidalVolumeL":vt,"inhaleFraction":i,"pauseAfterInhaleFraction":pi,"exhaleFraction":e,"pauseAfterExhaleFraction":pe,"route":route,"source":src,"reference":ref}
def bind_pose(pose,regions,emitters,activity,breathing,sun=0.0,reg=None,work=None,ground=None,extra_emit=None):
    """pose: pose dict, mutated; reg: dict regionId -> dict overrides."""
    reg=reg or {}; rb={}
    for r in regions:
        b={"exposedFraction":1.0,"sunlitFraction":sun if r not in ("shoes",) else sun}
        b.update(reg.get(r['id'],{}) if isinstance(r,dict) else reg.get(r,{}))
        rb[r['id']]=b
    th={"activityMet":activity,"breathing":breathing,"regions":rb,"emitters":{e:(extra_emit or {}).get(e,1.0) for e in emitters}}
    if work is not None: th["externalWorkW"]=work
    pose["thermal"]=th
