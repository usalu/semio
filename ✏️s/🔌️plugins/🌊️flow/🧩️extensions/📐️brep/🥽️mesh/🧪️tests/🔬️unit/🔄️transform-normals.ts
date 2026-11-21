import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Matrix3,Matrix4,Triangle,Vector3} from "three";
import fixture from "../🧫️fixtures/🔄️transform-normals.json";

const near=(actual:number[],expected:number[])=>actual.forEach((value,axis)=>expect(value).toBeCloseTo(expected[axis]!,6));

test("Three independently verifies reflected affine and selected rotation normals",()=>{
  const matrix=new Matrix4().fromArray(fixture.affine.matrix);
  const inverseTranspose=new Matrix3().getNormalMatrix(matrix);
  const rotation=new Matrix4().makeRotationAxis(new Vector3().fromArray(fixture.selection.axis),fixture.selection.angle);
  const positions=fixture.source.vertices.map(point=>new Vector3().fromArray(point).applyMatrix4(matrix));
  const outward=new Triangle(positions[0]!,positions[2]!,positions[1]!).getNormal(new Vector3());
  expect(matrix.determinant()).toBeLessThan(0);
  near(outward.toArray(),[0,0,1]);
  const authored=fixture.source.surface.filter(row=>row.kind==="vertexNormal").map(row=>row.normal!);
  const affine=authored.map(normal=>new Vector3().fromArray(normal).applyNormalMatrix(inverseTranspose).toArray());
  const rotated=new Vector3().fromArray(authored[0]!).applyNormalMatrix(new Matrix3().getNormalMatrix(rotation)).toArray();
  positions.forEach((point,index)=>near(point.toArray(),fixture.affine.vertices[index]!));
  affine.forEach((normal,index)=>near(normal,new Vector3().fromArray(fixture.affine.normalDirections[index]!).normalize().toArray()));
  near(rotated,fixture.selection.normalDirections[0]!);
  const path=process.env.SEMIO_TEST_TRANSFORM_NORMALS;
  if(!path)return;
  const output=JSON.parse(readFileSync(path,"utf8"));
  positions.forEach((point,index)=>near(output.affine.positions[index],point.toArray()));
  affine.forEach((normal,index)=>near(output.affine.normals[index],normal));
  near(output.affine.normals[2],outward.toArray());
  near(output.selection.normals[0],rotated);
  near(output.selection.normals[1],authored[1]!);
  near(output.selection.normals[2],outward.toArray());
  expect(output.affine.flipped).toBe(true);
  expect(output.selection.flipped).toBe(false);
  for(const result of [output.affine,output.selection]){
    expect(result.uv).toEqual(fixture.source.surface.filter(row=>row.kind==="cornerUv").map(row=>row.uv));
    expect(result.smooth).toBe(true);
    expect(result.seams).toEqual([0]);
  }
});
