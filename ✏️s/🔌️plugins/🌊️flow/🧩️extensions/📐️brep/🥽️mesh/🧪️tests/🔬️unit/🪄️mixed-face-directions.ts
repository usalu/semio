import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {Matrix3,Matrix4,Vector3} from "three";
import fixture from "../🧫️fixtures/🪄️mixed-face-directions.json";

const near=(actual:number[],expected:number[])=>actual.forEach((value,axis)=>expect(value).toBeCloseTo(expected[axis]!,6));

test("Three verifies mixed and uniform face direction bindings on disconnected selections",()=>{
  const pivot=new Vector3().fromArray(fixture.pivot);
  const authoredNormal=fixture.source.attributes.faceDirection.values[0]!;
  const authoredTangent=fixture.source.attributes.tangent.values[0]!;
  const transforms={
    rotation:new Matrix4().makeRotationAxis(new Vector3().fromArray(fixture.rotation.axis),fixture.rotation.angle),
    scale:new Matrix4().makeScale(fixture.scale.factor[0]!,fixture.scale.factor[1]!,fixture.scale.factor[2]!),
  };
  const path=process.env.SEMIO_TEST_MIXED_FACE_DIRECTIONS;
  const native=path?JSON.parse(readFileSync(path,"utf8")):undefined;
  for(const mode of ["rotation","scale"] as const) {
    const matrix=transforms[mode];
    const positions=fixture.source.vertices.map((point,index)=>{
      const value=new Vector3().fromArray(point);
      return (fixture.selection.includes(index)?value.sub(pivot).applyMatrix4(matrix).add(pivot):value).toArray();
    });
    near(positions[1]!,fixture[mode].partialPoint);
    const expectedNormal=new Vector3().fromArray(authoredNormal).applyNormalMatrix(new Matrix3().getNormalMatrix(matrix)).toArray();
    const expectedTangent=new Vector3().fromArray(authoredTangent).transformDirection(matrix).toArray();
    if(!native)continue;
    const result=native[mode];
    expect(result.faces).toEqual(fixture.source.faces);
    result.positions.forEach((point:number[],index:number)=>near(point,positions[index]!));
    expect(result.faceDirection[0]).toEqual(authoredNormal);
    expect(result.tangent[0]).toEqual(authoredTangent);
    near(result.faceDirection[1],expectedNormal);
    near(result.tangent[1],[...expectedTangent,authoredTangent[3]!]);
    expect(result.indicesSplit).toBe(true);
  }
});
