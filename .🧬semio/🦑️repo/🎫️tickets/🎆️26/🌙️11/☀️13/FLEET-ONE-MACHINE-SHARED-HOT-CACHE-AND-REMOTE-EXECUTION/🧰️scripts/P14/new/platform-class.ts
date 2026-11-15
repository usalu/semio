import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🌍️ Which platforms share records and requests, and which marker in a project's key text binds its record to the exact host. See `./🧬️schema/🔣️.json`. */
export const platformClassPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type PlatformClassPolicyV1=typeof platformClassPolicyV1;

/** 🧭️ The class tag of a platform (`darwin-arm64` and `linux-x64` are both `posix-shared`), the platform itself when it belongs to no class. */
export function platformClassV1(platform:string,policy:PlatformClassPolicyV1=platformClassPolicyV1):string{
 for(const [name,members] of Object.entries(policy.classes))if((members as readonly string[]).includes(platform))return name;
 return platform;
}

/** 🔒️ Whether the key text of a project names a declaration that makes its inferred record depend on the exact host. */
export const hostBoundV1=(keyText:string,policy:PlatformClassPolicyV1=platformClassPolicyV1):boolean=>policy.hostBound.some(marker=>keyText.includes(marker));

/** 🏷️ The platform tag a project's record is stored and looked up under: the class of the platform, or the exact platform for a host-bound project. */
export const recordPlatformV1=(keyText:string,platform:string,policy:PlatformClassPolicyV1=platformClassPolicyV1):string=>hostBoundV1(keyText,policy)?platform:platformClassV1(platform,policy);

/** 🤝️ Whether a machine of platform `server` serves the requests of a machine of platform `requester`: the same class. */
export const servesPlatformV1=(server:string,requester:string,policy:PlatformClassPolicyV1=platformClassPolicyV1):boolean=>platformClassV1(server,policy)===platformClassV1(requester,policy);
