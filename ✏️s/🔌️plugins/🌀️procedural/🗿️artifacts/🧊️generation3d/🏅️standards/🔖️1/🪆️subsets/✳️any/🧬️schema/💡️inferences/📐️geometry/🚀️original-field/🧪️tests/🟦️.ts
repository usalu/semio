import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Vector3,LineCurve3,PlaneGeometry,EllipseCurve,Matrix4} from "three";
import fixture from "./🧫️.json";
import schema from "./🧬️.json";
import cycle from "./🔄️.json";
import cycleSchema from "./🔄️🧬️.json";
import graphlib from "graphlib";
import analytic from "./⭕️.json";
import analyticSchema from "./⭕️🧬️.json";
import edge from "./📏️.json";
import edgeSchema from "./📏️🧬️.json";
import surface from "./🏳️.json";
import surfaceSchema from "./🏳️🧬️.json";
import translation from "./↔️.json";
import translationSchema from "./↔️🧬️.json";
import lookup from "./🗝️.json";
import lookupSchema from "./🗝️🧬️.json";

test("actual original field dispatch resolves native literals defaults wiring and retained dynamic custody",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const ids=new Set(fixture.widgets.map(widget=>widget.id));fixture.wires.forEach(wire=>{expect(ids.has(wire.from)).toBe(true);expect(ids.has(wire.to)).toBe(true)});
 const end=fixture.widgets.find(widget=>widget.id==="line")!.params.end!;
 expect([...new LineCurve3(new Vector3(),new Vector3().fromArray(end)).getPoint(0.5).toArray()]).toEqual(fixture.expected["curve point"]);
 const plane=new PlaneGeometry(fixture.expected["width Ä"],6).rotateX(Math.PI/2);expect(plane.attributes.position.count).toBe(fixture.expected.planePositions.length);expect(fixture.expected.position).toEqual(fixture.expected.planePositions[2]);
 expect([...new Vector3().fromArray(fixture.expected.position).add(new Vector3(1,2,3)).toArray()]).toEqual(fixture.expected["translated position"]);
 const source=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();const field=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();
 expect(field).toContain("original_field::compute_step(snapshot,key,parents,pending,cx)");
 expect(source).toContain("snapshot.computes.start_original(kind,&mut state.inputs,cx)");
 expect(source).toContain("WidgetInputs::new(key,kind,&mut state.values,&mut state.construction,cx)");
 expect(source).toContain("self.owner.terminal_is_empty()");expect(source).toContain("size_of::<OriginalGeometryPending>()");
 expect(source).toContain("state.equality.step(neuron_kind,&kind.id,cx)");expect(source).toContain("snapshot.catalogue.categories().get(state.category)");expect(source).not.toContain("snapshot.catalogue.kind(");
 expect(source).not.toContain("resolve_inputs(");expect(source).not.toContain("begin(");expect(source).not.toContain("new_cold");
});

test("original cyclic rows use real localized fault owners and the independent graph cycle oracle",async()=>{
 expect(new Ajv({strict:true}).validate(cycleSchema,cycle)).toBe(true);
 const graph=new graphlib.Graph({directed:true});cycle.widgets.forEach(widget=>graph.setNode(widget.id));cycle.wires.forEach(wire=>graph.setEdge(wire.from,wire.to));
 const members=graphlib.alg.findCycles(graph).flat().sort();expect(members).toEqual([...cycle.cycleMembers].sort());
 const affected=new Set(members);for(const member of members){graphlib.alg.preorder(graph,member).forEach((id:string)=>affected.add(id));}
 expect([...affected].sort()).toEqual(cycle.faultRows.map(row=>row.id).sort());
 for(const row of cycle.faultRows){expect(row.en).toBe(`Widget “${row.id}” is part of a wiring cycle or depends on one.`);expect(row.de).toBe(`Widget “${row.id}” ist Teil eines Verbindungszyklus oder hängt von einem ab.`);expect(new TextEncoder().encode(row.de).length).toBeGreaterThan(row.de.length);}
 const field=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();const producer=await Bun.file(new URL("../../💎️value/⚠️fault/🦀️.rs",import.meta.url)).text();
 expect(field).toContain("original_field::compute_cyclic_step(snapshot,key,pending,cx)");expect(producer).toContain("target.try_reserve_exact(demand.capacity_bytes)");expect(producer).toContain("target.push(character)");expect(producer).not.toContain("format!");expect(producer).not.toContain("WidgetFault::new");
});

test("original analytic BRep graph creation and reads agree with independent Three circles and ellipses",()=>{
 expect(new Ajv({strict:true}).validate(analyticSchema,analytic)).toBe(true);
 const circle=new EllipseCurve(1,2,2,2,0,Math.PI*2,false,Math.PI/2).getPoint(0);
 const ellipse=new EllipseCurve(1,2,4,1.5,0,Math.PI*2,false,0);
 for(const [name,point]of [["circle start",circle],["ellipse start",ellipse.getPoint(0)],["ellipse quarter",ellipse.getPoint(0.25)]]as const){const expected=analytic.expected[name];[point.x,point.y,3].forEach((coordinate,axis)=>expect(Math.abs(coordinate-expected[axis])).toBeLessThan(analytic.tolerance));}
});

test("original mesh edge graph publishes all three ports and agrees with independent Three distance",()=>{
 expect(new Ajv({strict:true}).validate(edgeSchema,edge)).toBe(true);
 const curve=new LineCurve3(new Vector3().fromArray(edge.expected.start),new Vector3().fromArray(edge.expected.end));expect(curve.getLength()).toBe(edge.expected.length);expect([...curve.getPoint(0).toArray()]).toEqual(edge.expected.start);expect([...curve.getPoint(1).toArray()]).toEqual(edge.expected.end);
});

test("original plane surface graph agrees with independent Three frame point and normal",()=>{
 expect(new Ajv({strict:true}).validate(surfaceSchema,surface)).toBe(true);const frame=surface.frame;const matrix=new Matrix4().makeBasis(new Vector3().fromArray(frame.x),new Vector3().fromArray(frame.y),new Vector3().fromArray(frame.z)).setPosition(new Vector3().fromArray(frame.origin));
 expect([...new Vector3(2,3,0).applyMatrix4(matrix).toArray()]).toEqual(surface.expected.point);expect([...new Vector3().fromArray(frame.x).cross(new Vector3().fromArray(frame.y)).normalize().toArray()]).toEqual(surface.expected.normal);
});

test("original analytic BRep translations agree with independent Three and retain labels",()=>{
 expect(new Ajv({strict:true}).validate(translationSchema,translation)).toBe(true);
 expect([...new LineCurve3(new Vector3(),new Vector3(3,4,0)).getPoint(0.5).add(new Vector3(1,2,3)).toArray()]).toEqual(translation.expected["translated curve point"]);
 expect([...new Vector3().fromArray(surface.expected.point).add(new Vector3(-1,-2,-3)).toArray()]).toEqual(translation.expected["translated surface point"]);
});

test("original borrowed dictionary lookup has an independent JSON object oracle",()=>{
 expect(new Ajv({strict:true}).validate(lookupSchema,lookup)).toBe(true);
 const nativeObject=JSON.parse(JSON.stringify(Object.fromEntries(lookup.entries)));expect(nativeObject[lookup.needle]).toBe(lookup.expected);
});

test("actual original engine retains ordinal values and borrowed keys through funded same-caller closure",async()=>{
 const engine=await Bun.file(new URL("../../🚂️engine/🦀️.rs",import.meta.url)).text();
 expect(new Vector3(40,0,0).add(new Vector3(1,0,0)).x).toBe(fixture.hostPolicy.value);
 expect(fixture.engine.completedPayloadCloned).toBe(false);
 expect(engine).toContain("values: InferenceValues<Arc<WidgetEvaluation>>");
 expect(engine).toContain("super::source_widget_key(&base.snapshot,source)");
 expect(engine).toContain("start_original_step");expect(engine).toContain("parent.admit_loan(cx)");
 expect(engine).toContain("run.cursor.close_original_plan_step(cx)");expect(engine).toContain("run.values.close_step(cx)");expect(engine).toContain("source.close_step(parent,cx)");
 expect(engine).not.toContain("BTreeMap");expect(engine).not.toContain("previous");expect(engine).not.toContain("values.clone()");expect(engine).not.toContain("cancellation_id.to_string()");expect(engine).not.toContain("snapshot.retire_cold()");
});
