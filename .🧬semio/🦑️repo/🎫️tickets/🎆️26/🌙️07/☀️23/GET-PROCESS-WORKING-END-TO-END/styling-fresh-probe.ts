/** 🔬️ Probes whether the generated styling artifacts are fresh without writing them. */
const { checkStylingArtifacts } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts");
try { checkStylingArtifacts(); console.log("FRESH"); } catch (e) { console.log("STALE:", String(e).slice(0, 300)); }
