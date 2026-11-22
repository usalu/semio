"""📐 Match the expansion-valve plan gauges and retain the boiler collar support rings."""
from copy import deepcopy

def boiler(a):
    fine=a['volumes']['50']
    stock={q['id']:q for q in fine['structure']}
    if any(q['id']=='member-collar-spigot' for q in a['volumes']['500']['structure']):return
    for k in ('100','200','500'):
        v=a['volumes'][k]
        drop=('member-collar','member-flue-collector','joint-collar-flange','member-hx-brackets')
        v['structure']=[q for q in v['structure'] if q['id'] not in drop]
        keep=['member-flue-collector','member-collar-spigot','member-collar-turret','member-hx-brackets-low']
        if k!='500':keep+=['joint-collar-flange','member-hx-brackets-high']
        v['structure']+=deepcopy([stock[id] for id in keep])
        v['decisions'].append(dict(en='The two concentric flue supports retain the 74 mm central bore through an 80 mm spigot. The collector tube meets the real spigot at the top of the casing. Folded steel brackets support the heat exchanger.',de='Die konzentrischen Abgasstützen behalten die 74-mm-Bohrung durch den 80-mm-Stutzen. Das Sammelrohr trifft den Stutzen auf der Gehäuseoberseite. Gekantete Stahlwinkel tragen den Wärmetauscher.'))
