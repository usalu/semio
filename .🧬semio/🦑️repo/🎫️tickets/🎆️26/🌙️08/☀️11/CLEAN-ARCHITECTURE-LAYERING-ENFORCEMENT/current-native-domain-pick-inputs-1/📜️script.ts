import { createHash } from "node:crypto";
import { dirname, resolve } from "node:path";
import { existsSync } from "node:fs";
if (process.argv[2] !== "verify") throw Error("Expected verify");
let root = import.meta.dir;
while (!existsSync(resolve(root, "Cargo.toml"))) {
    const parent = dirname(root);
    if (parent === root) throw Error("Repository root unavailable");
    root = parent;
}
const owner = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas";
const source = resolve(root, owner, "🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"), fixture = resolve(root, owner, "🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json");
const bodies = await Promise.all([Bun.file(source).text(), Bun.file(fixture).text()]);
const oracle = bodies[0].match(/let oracle = r#"(import\{PerspectiveCamera,Vector3,BufferGeometry,Float32BufferAttribute,Mesh,MeshBasicMaterial,DoubleSide,Raycaster,LineSegments,LineBasicMaterial\}from'three';[^\n]+)"#;/)?.[1];
if (!oracle) throw Error("Exact original owner oracle missing");
const world = JSON.parse(bodies[1]).world3d;
const child = Bun.spawn([process.execPath, "-e", oracle, JSON.stringify(world)], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
if (status !== 0) throw Error(stderr);
const expected = world.expect.instanceIds[0].split("#")[0];
if (stdout !== expected) throw Error(`Original solid oracle ${stdout} differs from ${expected}`);
const held = bodies.map(body => createHash("sha256").update(body).digest("hex"));
const live = await Promise.all([Bun.file(source).text(), Bun.file(fixture).text()]);
if (live.some((body, index) => createHash("sha256").update(body).digest("hex") !== held[index])) throw Error("Original oracle input advanced during verification");
const receipt = { ready: true, target: stdout, point: world.expect.pickWorldPosition, bindings: [source, fixture].map((path, index) => ({ path, sha256: held[index] })), independent: "Installed Three Raycaster", nativeExecuted: false, atomicSourceClaim: false };
await Bun.write(resolve(import.meta.dir, "../🗑️generated/current-native-canonical-6/product-domain-pick-oracle.json"), JSON.stringify(receipt, null, 2));
console.log(`[DEBUG] Original Product solid pick independentThree=true target=${stdout} point=${world.expect.pickWorldPosition}`);
