"""en1998 v9 step 8: compliance test helper and the numpy oracle for storey references. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
CT = S / "🧪️tests/⚖️compliance/🦀️.rs"
sub(CT, '(m.id, m.material, m.role) = (id.into(), "masonry".into(), "wall".into());', '(m.id, m.material, m.role) = (id.into(), "masonry".into(), semio_s_artifact_norm_en1998_model::MemberKind::Wall);')
sub(CT, "(m.wall_length_m, m.wall_thickness_m, m.wall_direction_rad, m.wall_storey_id) = (length, 0.3, direction, 0.0);", "(m.wall_length_m, m.wall_thickness_m, m.wall_direction_rad, m.wall_storey_id) = (length, 0.3, direction, String::new());")
O = ANY / "🔮️oracles/⚖️compliance/🐍️.py"
sub(O, '''    """🧱 Simple masonry wall-area densities by numpy: the wall rows (length, thickness, axis angle, storey with 0 = every storey) split by the dominant axis component per direction and storey over the floor area, and the governing minimum over both directions and the storeys."""
    import numpy as np

    length, thickness, angle, storey = np.array([(6.0, 0.3, 0.0, 1), (4.0, 0.3, 0.0, 1), (5.0, 0.3, math.pi / 2, 1), (3.0, 0.3, math.pi / 2, 0), (2.0, 0.25, 0.3, 2)]).T
    area, runs_y = 80.0, np.abs(np.sin(angle)) > np.abs(np.cos(angle))

    def density(along_y: bool, level: int) -> float:
        mask = ((storey == 0) | (storey == level)) & (runs_y == along_y)
        return float(np.sum(length[mask] * thickness[mask]) / area)
''', '''    """🧱 Simple masonry wall-area densities by numpy: the wall rows (length, thickness, axis angle, storey row id with "" = every storey) split by the dominant axis component per direction and storey row over the floor area, and the governing minimum over both directions and the storey rows."""
    import numpy as np

    rows = [(6.0, 0.3, 0.0, "s1"), (4.0, 0.3, 0.0, "s1"), (5.0, 0.3, math.pi / 2, "s1"), (3.0, 0.3, math.pi / 2, ""), (2.0, 0.25, 0.3, "s2")]
    length, thickness, angle = np.array([r[:3] for r in rows]).T
    storey = np.array([r[3] for r in rows])
    area, runs_y = 80.0, np.abs(np.sin(angle)) > np.abs(np.cos(angle))

    def density(along_y: bool, level: int) -> float:
        mask = ((storey == "") | (storey == f"s{level}")) & (runs_y == along_y)
        return float(np.sum(length[mask] * thickness[mask]) / area)
''')
flush()
