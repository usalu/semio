
if(process.argv[2]==="owner"){
 for(const path of ["🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DASHBOARD-TUI-WORKFORCE/🧪️native-publication/📓️execution.ts.md"])edit(path,text=>text.replace('cacheDirectory: string, options: ArtifactPublicationOptions','cacheDirectory: string, owner: string, options: ArtifactPublicationOptions').replace('stageArtifacts(directory,"immutable-executable",','stageArtifacts(directory,owner,'));
 edit("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts",text=>text.replace('basename(executable)),{signal,','basename(executable)),"native-execution",{signal,'));
 edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts",text=>text.replace('join(cache,"tools/dashboard-cli"),{signal:','join(cache,"tools/dashboard-cli"),"dashboard-execution",{signal:'));
}
