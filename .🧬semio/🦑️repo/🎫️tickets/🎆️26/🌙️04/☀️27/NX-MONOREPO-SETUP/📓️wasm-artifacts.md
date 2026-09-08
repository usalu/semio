# WASM Artifact Restoration

The actor WASM target returned a warm Nx cache hit. Its entire `pkg` output was moved into ticket evidence, then restored through Nx without recompilation. Every restored file matched its original SHA-256. A separate Node consumer initialized the restored WASM, created a KernelHost, read nonempty metrics, and freed it.

Platform: macOS arm64. Native compiler state remained available, but the restoring task used the Nx cache.

```json
{
  "framework_actor_bg.wasm": "3970514ceb630a4858e0894706cc03b1d8cb97033551544f5d22fb1e06c117f5",
  ".gitignore": "684888c0ebb17f374298b65ee2807526c066094c701bcc7ebbe1c1095f494fc1",
  "package.json": "947ba2fa9439cb9e5a94a582bdb2ac7fd959cfbbc714e3a3dc18c23089e6f0c3",
  "framework_actor_bg.wasm.d.ts": "baceb6fe4977a81834cde5b33faad73def1d03f6a344cbccfd398a131f0ce9b2",
  "framework_actor.d.ts": "c041f9c1ea6e1ec5493cf0e33ea97da9cfe929c7eba23ac43255f429ce5b248e",
  "framework_actor.js": "10416b4308d00e91769b0202cd028fff3836dff632466f67e4011aff476f821e"
}
```
