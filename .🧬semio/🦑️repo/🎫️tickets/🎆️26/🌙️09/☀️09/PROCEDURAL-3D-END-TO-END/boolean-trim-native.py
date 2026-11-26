#!/usr/bin/env python3
"""🧵️ Verify the actual exact Boolean trim recipes using native geometry libraries."""
import itertools
import json
from pathlib import Path
import subprocess

root = Path.cwd()
ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/bridge-current/boolean-native"
cache = root / ".cache/b/r/m/tmp/flow-native-n25/debug/build"
brep = root / "🧰️framework/🔨️modules/🧊️3d/📐️brep"
contract = brep / "📸️representation/🏄️surface/🔀️curved-boolean"
topology = brep / "⚙️engine/🏗️solid-topology"
modules = [
    ("corner", contract / "🧲️spherical-corner-box/🦀️.rs"),
    ("torus", contract / "🍩️spherical-torus-cut/🦀️.rs"),
    ("corner_trims", topology / "🧲️spherical-box-boundary/🦀️.rs"),
    ("torus_trims", topology / "🍩️spherical-torus-boundary/🦀️.rs"),
]
code = "\n".join(f'#[path={json.dumps(str(path), ensure_ascii=False)}]\npub mod {name};' for name,path in modules)
source = output/"trim_law.rs"
source.write_text(code + """
extern crate self as semio_framework_3d_brep_surface;
pub use surface_base::Surface;
pub mod curved_boolean {
    pub use crate::{corner::SphericalCornerBox,torus::SphericalTorusCut};
}
use curved_boolean::{SphericalCornerBox, SphericalTorusCut};
use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge,PrimitiveTrim};
use semio_framework_3d_brep_vector::{Pnt3,matrix::Frame3};

fn shell<const N:usize>(
    rings: &[usize], vertices: impl Fn(usize)->Pnt3,
    edge: impl Fn(usize)->PrimitiveEdge, surface: impl Fn(usize)->Surface,
    trim: impl Fn(usize,usize)->PrimitiveTrim, inward: impl Fn(usize)->bool,
) {
    let mut uses=[(0,0);N];
    for (face,count) in rings.iter().copied().enumerate() {
        let support=surface(face);
        let mut begin=None;
        let mut tail=None;
        for member in 0..count {
            let trim=trim(face,member);
            let e=edge(trim.edge);
            assert_eq!(trim.range,e.range);
            let a=e.vertices[usize::from(!trim.forward)];
            let b=e.vertices[usize::from(trim.forward)];
            if let Some(tail)=tail {assert_eq!(a,tail);}
            begin.get_or_insert(a);
            tail=Some(b);
            for i in 0..=8 {
                let t=e.range.0+(e.range.1-e.range.0)*i as f64/8.;
                assert!((support.eval_pcurve(&trim.curve,t)-e.curve.eval(t)).norm()<1e-10,"face={face}, member={member}, t={t}");
            }
            assert!((vertices(e.vertices[0])-e.curve.eval(e.range.0)).norm()<1e-10);
            assert!((vertices(e.vertices[1])-e.curve.eval(e.range.1)).norm()<1e-10);
            if (e.curve.eval(e.range.0)-e.curve.eval((e.range.0+e.range.1)/2.)).norm()>1e-10 {
                uses[trim.edge].0+=1;
                uses[trim.edge].1+=usize::from(trim.forward!=inward(face));
            }
        }
        assert_eq!(begin,tail);
    }
    assert!(uses.into_iter().all(|t| t==(0,0)||t==(2,1)), "{uses:?}");
}

fn main() {
    let origin=Pnt3::new(0.,0.,0.);
    for (center,radius,dimensions,volume) in [
        (origin,1.2,[1.5;3],9.708450789637022),
        (Pnt3::new(2.,0.,0.),1.1,[2.,1.7,1.6],10.318369792249353),
    ] {
        let cuboid=SphericalCornerBox::new(center,radius,origin,dimensions,1e-7).unwrap();
        shell::<31>(&SphericalCornerBox::RINGS,|i|cuboid.vertex(i),|i|corner_trims::edge(cuboid,i),
            |i|corner_trims::surface(cuboid,i),|i,j|corner_trims::trim(cuboid,i,j),|_|false);
        assert!((cuboid.volume()-volume).abs()<1e-9,"volume: {}",cuboid.volume());
    }
    for (radius,volume) in [(2.2,37.84114261331553),(1.9,25.771867175762203)] {
        let cut=SphericalTorusCut::new(Frame3::WORLD,radius,2.,0.5,1e-7).unwrap();
        shell::<7>(&SphericalTorusCut::RINGS,|i|cut.vertex(i),|i|torus_trims::edge(cut,i),
            |i|torus_trims::surface(cut,i),|i,j|torus_trims::trim(cut,i,j),|i|i==2);
        assert!((cut.volume()-volume).abs()<1e-9,"volume: {}",cut.volume());
    }
    println!("verified four exact native trim recipes");
}
""")
artifacts = []
for name in ("vector","curve","construction","surface"):
    crate=f"semio_framework_3d_brep_{name}"
    choices=sorted((cache/crate.replace("_","-")).glob(f"*/out/lib{crate}-*.rlib"),key=lambda p:p.stat().st_mtime,reverse=True)
    assert choices, crate
    artifacts.append((crate, choices))
rustc = subprocess.check_output(["rustup","which","rustc"],text=True).strip()
record=json.loads((output.parent/"geometry-lib-command.json").read_text())["args"]
search=[value for value in record if value.startswith(("dependency=","native="))]
all_direct=sorted({str(path.parent) for _,paths in artifacts for path in paths})
search.extend(f"dependency={path}" for path in all_direct)
base=[rustc,"--edition=2021","-Awarnings",str(source),"-o",str(output/"trim-law")]+list(itertools.chain.from_iterable(("-L",path) for path in dict.fromkeys(search)))
def alias(name):
    return "surface_base" if name.endswith("_surface") else name

for libs in itertools.product(*(choices for _,choices in artifacts)):
    args=[*base, *itertools.chain.from_iterable(("--extern",f"{alias(name)}={path}","--extern",f"{alias(name)}={path.with_suffix('.rmeta')}") for (name,_),path in zip(artifacts,libs))]
    result=subprocess.run(args,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    if result.returncode==0:
        subprocess.run([output/"trim-law"],check=True)
        break
    if not any(k in result.stderr for k in ("error[E0308]", "error[E0463]", "possibly newer version")):
        raise SystemExit(result.stderr)
else:
    raise SystemExit(result.stderr)
