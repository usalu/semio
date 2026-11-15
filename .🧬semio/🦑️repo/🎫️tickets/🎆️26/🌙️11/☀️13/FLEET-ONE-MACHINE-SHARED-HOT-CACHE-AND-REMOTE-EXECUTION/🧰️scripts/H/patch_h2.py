import sys
sys.argv = [sys.argv[0]] + [a for a in sys.argv[1:]]
exec(open(__file__.replace("patch_h2.py", "patch_core.py"), encoding="utf8").read().split("# ---- dependency storage")[0])
ffu = "../../" + e(0x1F6F0, "fleet-fetch") + "/"
mc = ffu + e(0x1FA9E, "mirror") + "/" + TS
ST = BOOT + "/" + e(0x1F4E6, "dependencies") + "/" + e(0x1F4BE, "storage") + "/" + TS
# storage: refresh state, storeRoot, transport env on the bun install child only
sub(ST, 'const fleetRequest=()=>({context:fleetFetchContextV1(process.env,workspace),', 'await refreshFleetMirrorStateV1(fleetFetchContextV1(process.env,workspace)).catch(()=>undefined);const fleetRequest=()=>({context:fleetFetchContextV1(process.env,workspace),storeRoot:context.root,')
sub(ST, 'captureOwnedCommandResultV1(process.execPath,args,row.plan.directory,{control:phase,env:environment,output:', 'captureOwnedCommandResultV1(process.execPath,args,row.plan.directory,{control:phase,env:{...environment,...fleetMirrorTransportEnvironmentV1("bun",environment,{...process.env,SEMIO_WORKSPACE_ROOT:workspace})},output:')
s0 = open(ST, encoding="utf8").read(); first = s0.split("\n", 1)[0]
sub(ST, first + "\n", first + "\n" + f'import{{refreshFleetMirrorStateV1,fleetMirrorTransportEnvironmentV1}}from"{mc}";\n') if "fleetMirrorTransportEnvironmentV1}from" not in s0 else None
# managers: generic environment builder
MG = BOOT + "/" + e(0x1F4E6, "dependencies") + "/" + e(0x1F50C, "managers") + "/" + TS
s1 = open(MG, encoding="utf8").read(); first1 = s1.split("\n", 1)[0]
sub(MG, 'return Object.freeze({...output,...overrides});\n  },\n  runtimeIdentity(host){', 'return Object.freeze({...output,...fleetMirrorTransportEnvironmentV1(descriptor.id,output,ambient),...overrides});\n  },\n  runtimeIdentity(host){')
sub(MG, first1 + "\n", first1 + "\n" + f'import{{fleetMirrorTransportEnvironmentV1}}from"{ffu}{e(0x1FA9E, "mirror")}/{TS}";\n')
# playwright download host through the tool cache environment
EV = LIB + "/" + e(0x1F3C3, "process") + "/" + e(0x1F33F, "environment") + "/" + TS
s2 = open(EV, encoding="utf8").read(); first2 = s2.split("\n", 1)[0]
sub(EV, '  env.PLAYWRIGHT_BROWSERS_PATH ??= repoPlaywrightBrowsersPath(repoRoot);\n  return env;', '  env.PLAYWRIGHT_BROWSERS_PATH ??= repoPlaywrightBrowsersPath(repoRoot);\n  for (const [name, value] of Object.entries(fleetMirrorTransportEnvironmentV1("playwright", env, {...env, SEMIO_WORKSPACE_ROOT: env.SEMIO_WORKSPACE_ROOT ?? repoRoot}))) env[name] ??= value;\n  return env;')
sub(EV, first2 + "\n", first2 + "\n" + 'import {fleetMirrorTransportEnvironmentV1} from "../../' + e(0x26A1, "caching") + "/" + e(0x1F680, "bootstrap") + "/" + e(0x1F6F0, "fleet-fetch") + "/" + e(0x1FA9E, "mirror") + "/" + TS + '";\n')
# nx package: sha512 pin
NX = BOOT + "/" + e(0x1F6E0, "tools") + "/" + e(0x1F4E6, "sdk") + "/" + e(0x1F4E5, "source") + "/" + e(0x1F4E5, "acquisition") + "/" + TS
s3 = open(NX, encoding="utf8").read(); first3 = s3.split("\n", 1)[0]
sub(NX, 'response=await host.fetch("https://registry.npmjs.org/nx/-/nx-"+row[0].slice(3)+".tgz",{signal,redirect:"error"});', 'response=await(host===NX_PACKAGE_SOURCE_HOST_V1?pinnedFetchV1(fleetFetchContextV1(process.env,workspace),pinFromIntegrityV1("https://registry.npmjs.org/nx/-/nx-"+row[0].slice(3)+".tgz",row[3])!,{signal,redirect:"error"}):host.fetch("https://registry.npmjs.org/nx/-/nx-"+row[0].slice(3)+".tgz",{signal,redirect:"error"}));')
sub(NX, first3 + "\n", first3 + "\n" + 'import{fleetFetchContextV1,pinnedFetchV1,pinFromIntegrityV1}from"../../../../' + e(0x1F6F0, "fleet-fetch") + "/" + TS + '";\n')
for item in log: print(item[0][-50:], item[1][:8], "->", item[2][:8])
