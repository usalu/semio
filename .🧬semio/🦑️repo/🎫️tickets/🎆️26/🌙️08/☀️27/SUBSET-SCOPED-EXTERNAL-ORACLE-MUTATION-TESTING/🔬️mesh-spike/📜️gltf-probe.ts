import * as THREE from "three";
import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";
class ShimFileReader {
  result: unknown = null; onload: (() => void) | null = null; onloadend: (() => void) | null = null; onerror: ((e: unknown) => void) | null = null;
  private done() { this.onloadend?.(); this.onload?.(); }
  readAsArrayBuffer(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = b; this.done(); }, (e) => this.onerror?.(e)); }
  readAsDataURL(blob: Blob) { blob.arrayBuffer().then((b) => { this.result = `data:${blob.type||"application/octet-stream"};base64,${Buffer.from(b).toString("base64")}`; this.done(); }, (e) => this.onerror?.(e)); }
}
(globalThis as { FileReader?: unknown }).FileReader ??= ShimFileReader as unknown as typeof FileReader;
const g = new THREE.BoxGeometry(1,1,1);
const m = new THREE.Mesh(g, new THREE.MeshStandardMaterial({ roughness: 0.4 }));
console.log("[start] exporting tiny box");
const t = setTimeout(() => { console.log("[TIMEOUT] gltf callback never fired"); process.exit(2); }, 8000);
await new Promise<void>((res, rej) => new GLTFExporter().parse(m, (out) => { clearTimeout(t); console.log("[ok] gltf keys:", Object.keys(out as object).join(",")); res(); }, rej, {}));
process.exit(0);
