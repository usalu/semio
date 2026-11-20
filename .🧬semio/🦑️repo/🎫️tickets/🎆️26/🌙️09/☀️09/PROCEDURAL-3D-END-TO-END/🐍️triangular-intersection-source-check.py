"""Compile the actual triangle queries with the original exact vector predicates."""
import json
from pathlib import Path

brep = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
validation = brep / "💡️queries/✅validation"
source = (validation / "✖️triangular-intersection/🦀️.rs").read_text()
source = source[source.index("fn project("):source.index("impl TrianglePairs")]
predicates = (brep / "📸️representation/➡️vector/⚖️predicates/🦀️.rs").read_text()
predicates = predicates[predicates.index("use super::"):predicates.index("#[cfg(test)]")]
predicates = predicates.replace("Pnt2, Pnt3, Vec3", "Pnt2, Pnt3")
predicates = predicates[:predicates.index("pub fn sign_of_dot")].rstrip().rsplit("///", 1)[0].rstrip()
while predicates.splitlines()[-1].startswith(("///", "//")):
    predicates = predicates.rsplit("\n", 1)[0].rstrip()
fixture = json.loads((validation / "🧪️tests/🧫️fixtures/triangular-intersection.json").read_text())

print("""
#![allow(dead_code)]
#[derive(Clone, Copy)] struct Pnt2 {x:f64,y:f64}
#[derive(Clone, Copy)] struct Pnt3 {x:f64,y:f64,z:f64}
impl Pnt2 {const fn new(x:f64,y:f64)->Self{Self{x,y}}}
impl Pnt3 {const fn new(x:f64,y:f64,z:f64)->Self{Self{x,y,z}}}
impl std::ops::Sub for Pnt3 {type Output=Self;fn sub(self,rhs:Self)->Self{Self::new(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z)}}
mod predicates {
""")
print(predicates)
print("} mod query { use super::*; use predicates::{Orient,orient2d,orient3d};")
print(source)
print("} use query::triangles_intersect;")
print("#[test] fn neutral_cases(){")

def points(rows):
    return "[" + ",".join("Pnt3::new(" + ",".join(repr(float(x)) for x in row) + ")" for row in rows) + "]"

print("let a=" + points(fixture["triangle"]) + ";")
for row in fixture["cases"]:
    print("let b=" + points(row["triangle"]) + ";")
    expected = str(row["intersects"]).lower()
    print(f"assert_eq!(triangles_intersect(a,b),{expected});")
    print(f"assert_eq!(triangles_intersect(b,a),{expected});")
    print(f"assert_eq!(triangles_intersect([a[1],a[0],a[2]],b),{expected});")
print("}")
