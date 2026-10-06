
/** 🧊️ Acquires immutable executable bytes before releasing the publisher's shared lease. */
export async function pinExecutableArtifact(source: string, cacheDirectory: string, owner: string, options: ArtifactPublicationOptions): Promise<string> {
  if (!isAbsolute(source) || !isAbsolute(cacheDirectory) || !isAbsolute(options.leaseDirectory)) throw Error("Explicit absolute executable publication paths required");
  const signal=options.signal??new AbortController().signal;
  const lease=await acquireResourceLease({directory:options.leaseDirectory,resource:`artifact:${dirname(source)}`,mode:"shared",signal,onWait:options.onWait});
  try {
    const digest=await executableDigest(source,signal),name=basename(source),directory=join(cacheDirectory,digest);
    await stageArtifacts(directory,owner,new Map([[name,source]]),options);
    const executable=join(directory,name);
    if(await executableDigest(executable,signal)!==digest)throw Error("Executable artifact changed during acquisition");
    return executable;
  } finally {lease.release();}
}

/** 🔏️ Hashes executable bytes in bounded cancellable reads. */
async function executableDigest(path: string, signal: AbortSignal): Promise<string> {
  const file=await open(path,"r"),hash=createHash("sha256"),bytes=Buffer.allocUnsafe(65536);
  try {
    for(;;){signal.throwIfAborted();const {bytesRead}=await file.read(bytes,0,bytes.length,null);signal.throwIfAborted();if(!bytesRead)return hash.digest("hex");hash.update(bytes.subarray(0,bytesRead));}
  } finally {await file.close();}
}
