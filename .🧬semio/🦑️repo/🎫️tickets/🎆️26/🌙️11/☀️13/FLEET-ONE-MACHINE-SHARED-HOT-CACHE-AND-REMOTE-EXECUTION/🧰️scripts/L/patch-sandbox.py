B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🔒️sandbox/"
def edit(path,pairs):
    s=open(B+path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:70])
        s=s.replace(old,new)
    open(B+path,'w',encoding='utf8').write(s)
edit('🟦️.ts',[
('import {accessSync,constants,realpathSync} from "node:fs";','import {accessSync,constants,lstatSync,readlinkSync,realpathSync} from "node:fs";'),
('''  '(deny file-read-data (subpath "/Users") (subpath "/Volumes") (subpath "/private/tmp") (subpath "/private/var/folders") (subpath "/private/var/tmp"))',''','''  `(deny file-read-data ${DARWIN_HIDDEN.map(path=>`(subpath "${path}")`).join(" ")})`,'''),
('''const quote=(value:string):string=>''','''const DARWIN_HIDDEN=["/Users","/Volumes","/private/tmp","/private/var/folders","/private/var/tmp","/private/etc/semio","/private/var/lib","/private/var/root","/private/var/db","/private/var/at","/private/etc/ssh","/Library/Keychains","/System/Volumes/Data/Users","/System/Volumes/Data/Volumes","/System/Volumes/Data/private/etc/semio","/System/Volumes/Data/private/var/lib","/System/Volumes/Data/private/var/root","/System/Volumes/Data/private/var/db"];
const LINUX_DIRECTORIES=["/usr","/bin","/sbin","/lib","/lib32","/lib64","/libx32","/etc/alternatives","/etc/ld.so.conf.d"],LINUX_FILES=["/etc/ld.so.cache","/etc/ld.so.conf","/etc/passwd","/etc/group"];

/** 🗺️ What the host has at one path: a directory, a file, a symbolic link (with its target) or nothing. */
export type RemoteExecutionHostEntryV1=Readonly<{kind:"directory"|"file"}|{kind:"symlink";target:string}>|undefined;

const hostEntry=(path:string):RemoteExecutionHostEntryV1=>{try{const info=lstatSync(path);return info.isSymbolicLink()?{kind:"symlink",target:readlinkSync(path)}:{kind:info.isDirectory()?"directory":"file"};}catch{return undefined;}};

const quote=(value:string):string=>'''),
('''/** 🧊️ The bubblewrap command line of the Linux sandbox: a read-only view of the host, the input root writable, fresh devices and process table, no network namespace access unless asked for, and everything dies with the parent. */
export function remoteExecutionBubblewrapArgumentsV1(spec:RemoteExecutionSandboxSpecV1):string[]{
 return ["bwrap",...(spec.network?[]:["--unshare-net"]),"--unshare-pid","--unshare-ipc","--die-with-parent","--new-session","--ro-bind","/","/","--dev","/dev","--proc","/proc","--bind",spec.root,spec.root,''','''/** 🧊️ The bubblewrap command line of the Linux sandbox: NOT the host. Only the system directories and libraries a toolchain needs (read-only, symbolic links such as `/bin` kept as links), the directories the action's tools declare readable, a private `/tmp`, fresh devices and process table, the input root writable, no network unless asked for, and everything dies with the parent. Home directories, `~/.ssh`, the worktree, the store and every credential file simply do not exist inside. */
export function remoteExecutionBubblewrapArgumentsV1(spec:RemoteExecutionSandboxSpecV1,host:(path:string)=>RemoteExecutionHostEntryV1=hostEntry):string[]{
 const system=[...LINUX_DIRECTORIES,...LINUX_FILES].flatMap(path=>{const entry=host(path);return entry===undefined?[]:entry.kind==="symlink"?["--symlink",entry.target,path]:["--ro-bind",path,path];});
 return ["bwrap",...(spec.network?[]:["--unshare-net"]),"--unshare-pid","--unshare-ipc","--die-with-parent","--new-session",...system,"--dev","/dev","--proc","/proc","--tmpfs","/tmp",...(spec.readable??[]).flatMap(path=>["--ro-bind",path,path]),"--bind",spec.root,spec.root,'''),
])
t=open(B+'🧪️tests/🟦️.ts',encoding='utf8').read()
old='''test("the macOS profile denies network and writes by default, escapes awkward paths, and the Linux command line binds only the input root",()=>{'''
assert t.count(old)==1
t=t.replace(old,'''test("the macOS profile denies network and writes by default, escapes awkward paths, and the Linux command line binds no host: only system directories, declared tools and the input root",()=>{''')
old2='''expect(bwrap.join(" ")).toContain("--ro-bind / / --dev /dev --proc /proc --bind /r /r --chdir /r/work --clearenv --setenv A 1 --setenv B 2 -- /bin/tool x");'''
assert t.count(old2)==1
t=t.replace(old2,'''expect(bwrap.join(" ")).toContain("--dev /dev --proc /proc --tmpfs /tmp --bind /r /r --chdir /r/work --clearenv --setenv A 1 --setenv B 2 -- /bin/tool x");
 const layout=(path:string)=>path==="/bin"||path==="/lib"?{kind:"symlink" as const,target:"usr"+path}:["/usr","/lib64","/etc/ld.so.cache"].includes(path)?{kind:path==="/etc/ld.so.cache"?"file" as const:"directory" as const}:undefined;
 const confined=remoteExecutionBubblewrapArgumentsV1({argv:["/bin/tool"],env:{},cwd:"/r",root:"/r",network:false,readable:["/home/u/.rustup/toolchains/t"]},layout);
 for(const secret of["/home","/root","/etc/shadow","/var","/mnt","/Users","/srv"])expect(confined.filter(item=>item===secret||item.startsWith(secret+"/")&&!item.startsWith("/home/u/.rustup")),secret).toEqual([]);
 expect(confined.join(" ")).toContain("--ro-bind /usr /usr --symlink usr/bin /bin --symlink usr/lib /lib --ro-bind /lib64 /lib64 --ro-bind /etc/ld.so.cache /etc/ld.so.cache");
 expect(confined.join(" ")).toContain("--ro-bind /home/u/.rustup/toolchains/t /home/u/.rustup/toolchains/t --bind /r /r");
 for(let index=0;index<confined.length;index++)if(confined[index]==="--ro-bind"||confined[index]==="--bind")expect(confined[index+1],"never the whole host").not.toBe("/");
 for(const hidden of["/Users","/private/etc/semio","/private/var/root","/Library/Keychains","/System/Volumes/Data/Users"])expect(profile).toContain(`(subpath "${hidden}")`);''')
open(B+'🧪️tests/🟦️.ts','w',encoding='utf8').write(t)
print("sandbox ok")
