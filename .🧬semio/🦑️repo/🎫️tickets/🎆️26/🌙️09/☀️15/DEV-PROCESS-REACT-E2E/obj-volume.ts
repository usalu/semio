import { Mesh, Vector3 } from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";

export function objVolume(source: string): number {
  let volumeTimesSix = 0;
  const a = new Vector3(), b = new Vector3(), c = new Vector3();
  new OBJLoader().parse(source).traverse(node => {
    if (!(node instanceof Mesh)) return;
    const positions = node.geometry.getAttribute("position");
    const index = node.geometry.getIndex();
    const positionIndex = (i: number) => index?.getX(i) ?? i;
    for (let i = 0; i + 2 < (index?.count ?? positions.count); i += 3) {
      a.fromBufferAttribute(positions, positionIndex(i));
      b.fromBufferAttribute(positions, positionIndex(i + 1));
      c.fromBufferAttribute(positions, positionIndex(i + 2));
      volumeTimesSix += a.dot(b.cross(c));
    }
  });
  return Math.abs(volumeTimesSix / 6);
}
