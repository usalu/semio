import { test } from "node:test";
import assert from "node:assert/strict";
import { PNG } from "pngjs";
import { paintEvidence } from "./🔍️wgpu-paint-oracle.mjs";

test("PNG pixel witness requires a reversible local change", () => {
  const image = new PNG({width:16,height:16}); image.data.fill(40);
  const before=PNG.sync.read(PNG.sync.write(image)).data;
  const active=Buffer.from(before); for(let i=0;i<40*4;i+=4)active[i]=200;
  const after=Buffer.from(before);
  assert.equal(paintEvidence(before,active,after).pass,true);
  assert.equal(paintEvidence(before,before,after).pass,false);
  assert.equal(paintEvidence(before,active,active).pass,false);
  const drift=Buffer.from(after); for(let i=0;i<40*4;i+=4)drift[i]=100;
  assert.equal(paintEvidence(before,active,drift).pass,false);
  assert.throws(()=>paintEvidence(before,active,after.subarray(1)));
});
