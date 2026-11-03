from kit import *

NM = {'n': 'near', 'f': 'far'}


def seg(bone, l0, l1, w0, w1, reg, label):
    return dict(bone=bone, l0=l0, l1=l1, w0=w0, w1=w1, reg=reg, label=label)


def group_of(chain, bind, idxs, label, reg=None):
    first, last = chain[idxs[0]], chain[idxs[-1]]
    regs = []
    for i in idxs:
        r = chain[i]['reg']
        for x in (r if isinstance(r, list) else [r]):
            if x not in regs:
                regs.append(x)
    return dict(bind=chain[bind]['bone'], first=first, last=last, label=label, reg=reg or (regs if len(regs) > 1 else regs[0]), terminal=idxs[-1] == len(chain) - 1)


def groups_for(chain, T, plan_groups):
    if T <= 100:
        return [group_of(chain, i, [i], c['label']) for i, c in enumerate(chain)]
    return [group_of(chain, b, ix, lb) for b, ix, lb in plan_groups[200 if T == 200 else 500]]


def leg_front(k, T, side, sg, chain, plan_groups, z, mid=1.0, ground=-.005, thin=1.0):
    """front columns of one leg chain; side 'n' or 'f'; chain bones carry a {s} placeholder."""
    for g in groups_for(chain, T, plan_groups):
        f, l = g['first'], g['last']
        bone = g['bind'].format(s=side)
        ob = f['bone'].format(s=side)
        tb = l['bone'].format(s=side)
        y0 = k.o(ob)[1]
        y1 = k.t(tb)[1] + (ground if g['terminal'] else 0)
        k.col(f"{NM[side]}-{g['label']}", bone, sg * f['l0'], sg * l['l1'], f['w0'] * thin, l['w1'] * thin, g['reg'], z, y0=y0, y1=y1)


def leg_plan(k, T, side, sg, chain, plan_groups, z, toe=.025, thin=1.0):
    for g in groups_for(chain, T, plan_groups):
        f, l = g['first'], g['last']
        bone = g['bind'].format(s=side)
        ob = f['bone'].format(s=side)
        tb = l['bone'].format(s=side)
        x0 = k.o(ob)[0]
        x1 = k.t(tb)[0] + (toe if g['terminal'] else 0)
        k.cap(f"{NM[side]}-{g['label']}", bone, sg * f['l0'], sg * l['l1'], f['w0'] * thin, l['w1'] * thin, g['reg'], z, x0=x0, x1=x1)


def text(en, de):
    return en, de
