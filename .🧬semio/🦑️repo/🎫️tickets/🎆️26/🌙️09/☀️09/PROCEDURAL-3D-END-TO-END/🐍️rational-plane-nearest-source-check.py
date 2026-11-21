"""📐️ Verify the production rational-patch inverse with native vectors and a shared Three.js fixture."""
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory

ticket=Path(__file__).parent
generated=ticket/"🗑️generated"
modeling=generated/"authored-intersection-native"
dependencies=generated/"mesh-inputs-9f403ac70c06ea23"
analysis=Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis").resolve()
fixture=json.loads((analysis/"🧪️tests/🧫️fixtures/rational-plane-nearest.json").read_text(encoding="utf-8"))
owner=analysis/"🎯️nearest-analytic/📐️rational-plane/🦀️.rs"

def rust(values):
    if isinstance(values,list):
        return "["+",".join(map(rust,values))+"]"
    return repr(float(values))

ring=[fixture["controls"][i][j] for i,j in [(0,0),(1,0),(1,1),(0,1)]]
weights=[fixture["weights"][i][j] for i,j in [(0,0),(1,0),(1,1),(0,1)]]
source = f"""
use semio_framework_3d_brep_vector::{{Pnt3,Vec3}};
#[path = {json.dumps(str(owner),ensure_ascii=False)}]
mod rational_plane;
const POINTS:[[f64;3];4]={rust(ring)};
const WEIGHTS:[f64;4]={rust(weights)};
fn point(p:[f64;3])->Pnt3{{Pnt3::from_array(p)}}

#[test] fn original_projection_rejects_unsupported_footprints(){{
 let mut folded=POINTS;
 folded.swap(1,2);
 let mut nonplanar=POINTS;
 nonplanar[2][2]+=0.25;
 let target=point([0.,0.,0.]);
 assert!(rational_plane::nearest_uv(folded.map(point),WEIGHTS,target).is_none());
 assert!(rational_plane::nearest_uv(nonplanar.map(point),WEIGHTS,target).is_none());
 assert!(rational_plane::nearest_uv(POINTS.map(point),[1.,0.,1.,1.],target).is_none());
}}

#[test]fn original_bilinear_recovers_unit_weight_parameters(){{
 let uv=rational_plane::nearest_uv(POINTS.map(point),[1.;4],point([0.,0.,0.])).unwrap();
 assert_eq!(uv,[0.,0.]);
}}
"""
for index,row in enumerate(fixture["queries"]):
    expected=[(p-a)/(b-a) for p,(a,b) in zip(row["uv"],fixture["domain"])]
    source+=f"""
#[test] fn nearest_{index}(){{
 let actual=rational_plane::nearest_uv(POINTS.map(point),WEIGHTS,point({rust(row["target"])})).unwrap();
 let expected={rust(expected)};
 assert!(actual.into_iter().zip(expected).all(|(a,b)|(a-b).abs()<1e-10),"{{actual:?}} != {{expected:?}}");
}}
"""

with TemporaryDirectory(dir=generated) as folder:
    driver=Path(folder)/"rational-plane.rs"
    executable=driver.with_suffix(".exe" if os.name=="nt" else "")
    driver.write_text(source,encoding="utf-8")
    launcher=["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20"]
    compiler=["rustc","--edition=2021","--test",str(driver),"-o",str(executable),"-L","dependency="+str(dependencies),
              "-L","dependency="+str(modeling),"--extern","semio_framework_3d_brep_vector="+str(modeling/"libsemio_framework_3d_brep_vector.rlib")]
    subprocess.run(launcher+compiler,check=True)
    subprocess.run(launcher+[str(executable)],check=True)
