import json,sys
V="️"
f="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧾️register-operators/🦀️.rs"
def fn(n,e,d,a,b): return {"name":n,"emoji":e,"doc":d,"from":a,"to":b}
def ch(e,n,d,funcs): return {"dir":e+V+n,"doc":f"{e}{V} {d} See [`super`].","funcs":funcs}
spec={"file":f,"function":"register","children":[
 ch("🧊","schemas-primitives-and-curves","Schemas, primitives and curves: the brep schemas with the deconstruct operator, the 3d primitive solids and the curve operators.",[
  fn("register_schemas_and_deconstruct","📦","Registers the geometry, topology, brep and text schemas and the brep deconstruct operator.",7,39),
  fn("register_primitive_solids","🧱","Registers the box, sphere, cylinder, cone, torus and convex hull operators.",41,101),
  fn("register_curves","〰","Registers the line, circle, arc, ellipse, polyline, rectangle, polygon, interpolate, approximate and helix operators.",103,212)]),
 ch("🪡","surfaces-and-sweeps","Surfaces and sweeps: the surface constructors and the extrude, revolve, loft, sweep, pipe and helical sweep operators.",[
  fn("register_surfaces","🛹","Registers the plane, planar face, nurbs grid, coons, offset and thicken surface operators.",214,286),
  fn("register_sweeps","🌀","Registers the solid extrude and the extrude, revolve, loft, sweep, pipe and helical sweep operators.",288,378)]),
 ch("🧮","booleans-transforms-and-solid-features","Booleans, transforms and solid features: the boolean, transform, pattern, fillet, chamfer, shell, draft and defeature operators.",[
  fn("register_booleans","➗","Registers the fuse, cut, intersect and compound cut operators.",380,405),
  fn("register_transforms_and_patterns","🔄","Registers the translate, rotate, scale, mirror, copy and pattern operators.",407,516),
  fn("register_solid_features","🛠","Registers the fillet, chamfer, shell, draft, offset solid and defeature operators.",518,643)]),
 ch("🔍","intersection-and-evaluation-operators","Intersection and evaluation operators: the section, split and intersection operators and the curve and surface evaluation operators.",[
  fn("register_intersections","✂","Registers the section, split, curve curve, curve surface and surface surface intersection operators.",645,713),
  fn("register_evaluations","📍","Registers the curve and surface point, tangent, domain, curvature, normal and closest parameter operators.",715,825)]),
 ch("📐","measurement-utility-and-topology-operators","Measurement, utility and topology operators: the measure operators, the vertex, sew, heal and nurbs conversion utilities and the shell, compound, explode and label topology operators.",[
  fn("register_measurements","📏","Registers the volume, area, length, center of mass, bounding box, distance, closest point, classify and validate operators.",827,902),
  fn("register_utilities","🧰","Registers the vertex, face from wire, sew, heal and convert to nurbs operators.",904,952),
  fn("register_topology","🕸","Registers the shells, compound, explode and label operators.",954,999)]),
 ch("📂","io-operators","IO operators: the step, stl, obj and dwg import and export operators.",[
  fn("register_io","💾","Registers the step, stl, obj and dwg export and import operators.",1001,1088)]),
]}
json.dump(spec,open(sys.argv[1],"w",encoding="utf8"),ensure_ascii=False,indent=1)
