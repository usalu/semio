import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import thin_proto as T, proto
from cases import CASES, i_section, channel, angle, tee, mono_i

def cl_i(b, h, tf, tw):
    c = b / 2
    return [(0, tf/2, b, tf/2, tf), (c, tf/2, c, h - tf/2, tw), (0, h - tf/2, b, h - tf/2, tf)]
def cl_channel(h, b, tf, tw):
    return [(b, tf/2, tw/2, tf/2, tf), (tw/2, tf/2, tw/2, h - tf/2, tw), (tw/2, h - tf/2, b, h - tf/2, tf)]
def cl_angle(a, b, t):
    return [(a, t/2, t/2, t/2, t), (t/2, t/2, t/2, b, t)]
def cl_tee(b, h, tf, tw):
    c = b/2
    return [(0, h - tf/2, b, h - tf/2, tf), (c, h - tf/2, c, 0, tw)]
cases = {
 "i-section": (cl_i(0.1, 0.12, 0.008, 0.006), CASES["i-section"]),
 "channel": (cl_channel(0.2, 0.075, 0.011, 0.0085), CASES["channel"]),
 "angle": (cl_angle(0.1, 0.1, 0.01), CASES["angle"]),
 "tee": (cl_tee(0.1, 0.1, 0.01, 0.008), CASES["tee"]),
}
for name, (w, (o, h)) in cases.items():
    t = T.analyse(w)
    f = proto.analyse(o, h, h_factor=3)
    rel = lambda a, b: (a - b) / abs(b)
    print(name, f"A {rel(t['area'], f['area']):+.2%} Iy {rel(t['ic'][0], f['ic'][0]):+.2%} J {rel(t['j'], f['j']):+.2%} Iw {rel(t['gamma'], f['gamma']):+.2%} sc {t['sc'][0]-f['sc'][0]:+.5f} {t['sc'][1]-f['sc'][1]:+.5f} Asx {rel(t['a_s'][0], f['a_s'][0]):+.2%} Asy {rel(t['a_s'][1], f['a_s'][1]):+.2%}")
