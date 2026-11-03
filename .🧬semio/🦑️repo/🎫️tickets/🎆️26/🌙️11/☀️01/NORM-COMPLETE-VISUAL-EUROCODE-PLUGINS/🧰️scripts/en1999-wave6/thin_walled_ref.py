"""EN 1999 v6 reference of the open thin-walled extrusion constants, third-party: sectionproperties (FE warping analysis, sharp corners) for I, channel and angle against the mechanics kernel `analyse_thin_walled` used by `part_1_1::thin_walled_constants`. Prints the numbers asserted by the Rust test `thin_walled_constants_reproduce_sectionproperties`. Usage: /home/deck/semio/.venv/bin/python -I thin_walled_ref.py"""
from sectionproperties.analysis import Section
from sectionproperties.pre.library import angle_section, channel_section, i_section

MM = 1e3


def solved(geometry, mesh=0.4):
    geometry.create_mesh(mesh_sizes=[mesh])
    s = Section(geometry)
    s.calculate_geometric_properties()
    s.calculate_warping_properties()
    return s


def report(name, s):
    cx, cy = s.get_c()
    sx, sy = s.get_sc()
    print(name, dict(It=s.get_j() / MM**4, Iw=s.get_gamma() / MM**6, sc_y=(sx - cx) / MM, sc_z=(sy - cy) / MM))


report("i 200x100", solved(i_section(d=200, b=100, t_f=10, t_w=6, r=0, n_r=1)))
report("channel 100x50", solved(channel_section(d=100, b=50, t_f=7, t_w=5, r=0, n_r=1)))
report("angle 60x6", solved(angle_section(d=60, b=60, t=6, r_r=0, r_t=0, n_r=1)))
