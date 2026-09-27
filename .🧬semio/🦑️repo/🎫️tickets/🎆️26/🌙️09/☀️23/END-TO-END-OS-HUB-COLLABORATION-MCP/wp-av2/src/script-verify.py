# -*- coding: utf-8 -*-
"""🧰️ AV2 one-off: inserts the `verify video-render-export [native]` lane into the overlay's root 📜️script.ts, right after the
`media-export-encoding` lane it mirrors."""
p = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/📜️script.ts"
t = open(p, encoding="utf-8").read()
anchor = '    if (segments[0] === "file-open-import") {\n'
assert t.count(anchor) == 1
lane = '''    if (segments[0] === "video-render-export") {
      const { testVideoRenderProgramContract } = await import("./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts");
      const { testVideoRenderJobContract } = await import("./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts");
      const { testRasterVideoAvcPcmContract } = await import("./🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts");
      const { testVideoRenderHostContract } = await import("./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts");
      testVideoRenderProgramContract();
      testVideoRenderJobContract();
      testRasterVideoAvcPcmContract();
      await testVideoRenderHostContract();
      // 🧬️ `tsc --strict` over the raster video twin alone: it is self-contained by construction, so this lane's verdict
      // never inherits the renderer graph's unrelated diagnostics. FFmpeg's reading of the same streams is the repository
      // test platform's `🖌️raster/🎥️video/🧪️tests/🎞️ffmpeg-decode` case (`bun ./📜️script.ts test parity`).
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🔨️modules/🖌️raster/🎥️video")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "video_render", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-raster", "--lib", "video", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "wire_effect_round_trip", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "export_video", "--", "--nocapture"], this.root);
      }
      return;
    }
'''
t = t.replace(anchor, lane + anchor, 1)
open(p, "w", encoding="utf-8").write(t)
print("inserted")
