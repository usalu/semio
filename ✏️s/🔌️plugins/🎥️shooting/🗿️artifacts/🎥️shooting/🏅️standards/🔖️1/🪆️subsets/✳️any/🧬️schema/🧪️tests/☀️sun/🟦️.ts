import { expect, test } from "bun:test";
import { DirectionalLight, Scene } from "three";
import fixture from "../🧫️fixtures/☀️sun/🔣️.json";

for (const scenario of fixture.cases) {
  test(`shooting sun request agrees with Three visible light traversal: ${scenario.name}`, () => {
    const scene = new Scene();
    const sun = new DirectionalLight("#ffffff", scenario.intensity);
    sun.visible = scenario.enabled;
    scene.add(sun);
    let intensity = 0;
    scene.traverseVisible(object => { if (object instanceof DirectionalLight) intensity += object.intensity; });
    expect(intensity).toBe(scenario.expected);
  });
}
