/** 🥖️ Carries the acquired Bun identity into every Nx producer and source watcher. */
export function pinnedNxRuntimeEnvironment(source:NodeJS.ProcessEnv,bun:string,platform:NodeJS.Platform=process.platform):NodeJS.ProcessEnv {
  const paths=platform==="win32"?win32:posix;
  if(!paths.isAbsolute(bun))throw new Error("Nx Bun runtime must be absolute");
  const environment={...source},keys=Object.keys(source).filter(key=>platform==="win32"?key.toUpperCase()==="PATH":key==="PATH"),key=keys.find(value=>value==="PATH")??keys[0]??"PATH",previous=source[key]??"";
  for(const candidate of keys)delete environment[candidate];
  const directory=paths.dirname(bun),entries=previous.split(paths.delimiter).filter(entry=>entry && (platform==="win32"?entry.toLowerCase()!==directory.toLowerCase():entry!==directory));
  environment[key]=[directory,...entries].join(paths.delimiter);
  return environment;
}
