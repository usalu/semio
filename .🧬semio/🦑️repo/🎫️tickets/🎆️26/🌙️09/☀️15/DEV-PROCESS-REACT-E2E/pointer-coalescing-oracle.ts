import assert from "node:assert/strict";
import { ReplaySubject, take } from "rxjs";
import { OrthographicCamera, Plane, Raycaster, Vector2, Vector3 } from "three";

const cases = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/engagement-pointer-order.json").json();
const view = new OrthographicCamera(-1, 1, 1, -1, 0.1, 10);
view.position.z = 2;
view.updateMatrixWorld();
const ray = new Raycaster();
const ground = new Plane(new Vector3(0, 0, 1), 0);
function worldX(pixel: number): number {
  ray.setFromCamera(new Vector2(pixel / 50 - 1, 0), view);
  return ray.ray.intersectPlane(ground, new Vector3())!.x;
}

for (const { id, steps, events } of cases) {
  const observed: [string, number][] = [];
  let queue = new ReplaySubject<number>(1);
  let busy = false;
  let last: number | undefined;
  const send = (x: number) => {
    if (last === x) return;
    observed.push(["worldPointerMove", x]);
    busy = true;
    last = x;
  };
  const clear = () => {
    queue.complete();
    queue = new ReplaySubject<number>(1);
    last = undefined;
  };
  for (const [step, pixel] of steps) {
    switch (step) {
      case "move": {
        const x = worldX(pixel);
        if (busy) queue.next(x);
        else send(x);
        break;
      }
      case "settle": {
        const ready = queue;
        queue = new ReplaySubject<number>(1);
        busy = false;
        ready.pipe(take(1)).subscribe(send);
        ready.complete();
        break;
      }
      case "select": clear(); break;
      case "down":
        clear();
        observed.push(["worldPointerDown", worldX(pixel)]);
        break;
    }
  }
  assert.deepEqual(observed, events, id);
}
console.log("[DEBUG] RxJS and Three.js matched the pointer publication fixture");
