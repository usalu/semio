pinned = '''        "bootstrapSha256": {
          "aarch64-apple-darwin": "aeb4105778ca1bd3c6b0e75768f581c656633cd51368fa61289b6a71696ac7e1",
          "x86_64-apple-darwin": "33cf85df9142bc6d29cbc62fa5ca1d4c29622cddb55213a4c1a43c457fb9b2d7",
          "aarch64-unknown-linux-gnu": "9732d6c5e2a098d3521fca8145d826ae0aaa067ef2385ead08e6feac88fa5792",
          "x86_64-unknown-linux-gnu": "4acc9acc76d5079515b46346a485974457b5a79893cfb01112423c89aeb5aa10",
          "aarch64-unknown-linux-musl": "88761caacddb92cd79b0b1f939f3990ba1997d701a38b3e8dd6746a562f2a759",
          "x86_64-unknown-linux-musl": "9cd3fda5fd293890e36ab271af6a786ee22084b5f6c2b83fd8323cec6f0992c1",
          "x86_64-pc-windows-msvc": "86478e53f769379d7f0ebfa7c9aa97cb76ca92233f79aa2cc0dbee2efaac73c7",
          "aarch64-pc-windows-msvc": "3af309e6c3062aa11df0e932954f69d13b734d8a431e593812f3ecd9ff9e6ef6"
        },
        "distributionManifestSha256": {
          "2026-07-20": "1bf70a4f1406732dd79957ccc0d0afc78bdc69cdfa3b5f57857ab56727e05d56"
        },
'''
sub(SCH, '        "bootstrapVersion": "1.29.0",\n        "downloadProgressBytes": 4194304,', '        "bootstrapVersion": "1.29.0",\n' + pinned + '        "downloadProgressBytes": 4194304,')
BS = R + "/" + e(0x1F4E5, "bootstrap") + "/" + e(0x1F7E6, ".ts")
sub(BS, 'import {linuxHostLibcV1} from "../../../../../🎛️commands/🔧️setup/🟦️.ts";\n', 'import {linuxHostLibcV1} from "../../../../../🎛️commands/🔧️setup/🟦️.ts";\n' + imp(3))
sub(BS, 'export function downloadRustupBytesV1(url:string,maximum:number,control:ResourceLeaseControl,consume:(bytes:Uint8Array)=>Promise<void>):Promise<number>{', 'export function downloadRustupBytesV1(url:string,maximum:number,control:ResourceLeaseControl,consume:(bytes:Uint8Array)=>Promise<void>,pin?:Readonly<{sha256:string}>):Promise<number>{')
sub(BS, 'const response=await fetch(url,{signal:AbortSignal.any([control.signal,controller.signal]),redirect:"error"});', 'const response=await(pin?pinnedFetchV1(fleetFetchContextV1(),{url,sha256:pin.sha256},{signal:AbortSignal.any([control.signal,controller.signal]),redirect:"error"}):fetch(url,{signal:AbortSignal.any([control.signal,controller.signal]),redirect:"error"}));')
sub(BS, 'parts:Uint8Array[]=[];await downloadRustupBytesV1(url+".sha256",1024,control,async bytes=>{parts.push(bytes);});const expected=Buffer.concat(parts).toString("utf8").match(/^([a-f0-9]{64})(?:\\s|$)/u)?.[1];if(!expected)throw Error("Pinned Rustup bootstrap checksum required");', 'expected=(schema.definitions.Protocol.const.bootstrapSha256 as Readonly<Record<string,string>>)[host];if(!expected)throw Error("Pinned Rustup bootstrap checksum required");')
sub(BS, 'position+=written.bytesWritten;}});await file.sync();', 'position+=written.bytesWritten;}},{sha256:expected});await file.sync();')
IN = R + "/" + e(0x1F4E5, "installation") + "/" + e(0x1F7E6, ".ts")
s2 = open(IN, encoding="utf8").read()
first2 = s2.split("\n", 1)[0]
sub(IN, 'async function responseBytes(url:string,maximum:number,control:ResourceLeaseControl):Promise<Buffer>{const chunks:Uint8Array[]=[];await downloadRustupBytesV1(url,maximum,control,async bytes=>{chunks.push(bytes);});return Buffer.concat(chunks);}', 'async function responseBytes(url:string,maximum:number,control:ResourceLeaseControl,pin?:Readonly<{sha256:string}>):Promise<Buffer>{const chunks:Uint8Array[]=[];await downloadRustupBytesV1(url,maximum,control,async bytes=>{chunks.push(bytes);},pin);return Buffer.concat(chunks);}')
sub(IN, 'checksum=(await responseBytes(url+".sha256",1024,control)).toString("utf8").match(/^([a-f0-9]{64})(?:\\s|$)/u)?.[1];if(!checksum)throw Error("Pinned distribution manifest checksum required");const bytes=await responseBytes(url,protocol.maximumDistributionManifestBytes,control);', 'pinnedManifest=(protocol.distributionManifestSha256 as Readonly<Record<string,string>>)[date],checksum=pinnedManifest??(await responseBytes(url+".sha256",1024,control)).toString("utf8").match(/^([a-f0-9]{64})(?:\\s|$)/u)?.[1];if(!checksum)throw Error("Pinned distribution manifest checksum required");const bytes=await responseBytes(url,protocol.maximumDistributionManifestBytes,control,pinnedManifest?{sha256:pinnedManifest}:undefined);')
sub(IN, 'await downloadRustupBytesV1(row.url,protocol.maximumArchiveBytes,control,async bytes=>{hash.update(bytes);await file.writeFile(bytes);});', 'await downloadRustupBytesV1(row.url,protocol.maximumArchiveBytes,control,async bytes=>{hash.update(bytes);await file.writeFile(bytes);},{sha256:row.digest});')
sub(IN, first2 + "\n", first2 + "\n" + imp(3))
P = FW + "/" + e(0x1F6CD, "products") + "/" + e(0x1F4D3, "print") + "/" + e(0x1F4E6, "artifacts") + "/" + e(0x1F4E5, "material") + "/" + e(0x1F4E4, "producer") + "/" + e(0x1F7E6, ".ts")
ffp = "../../../../" + e(0x1F991, "repo") + "/" + e(0x1F528, "modules") + "/" + e(0x1F4DA, "library") + "/" + e(0x26A1, "caching") + "/" + e(0x1F680, "bootstrap") + "/" + e(0x1F6F0, "fleet-fetch") + "/" + e(0x1F7E6, ".ts")
s3 = open(P, encoding="utf8").read()
first3 = s3.split("\n", 1)[0]
sub(P, first3 + "\n", first3 + "\n" + 'import{pinnedFetchV1,fleetBlobBytesV1,fleetBlobPublishBytesV1,sharedFleetFetchContextV1}from"' + ffp + '";\n')
sub(P, 'response=await fetchPrintHttpsV1(distribution.url,sink.control,{});', 'response=await fetchPrintHttpsV1(distribution.url,sink.control,{},(url,options)=>pinnedFetchV1(sharedFleetFetchContextV1(context.workspace),{url,sha256:distribution.sha256},options as unknown as RequestInit));')
sub(P, 'const range=`${file.offset}-${file.offset+file.bytes-1}`,response=await fetchPrintHttpsV1(bundle.url,sink.control,{Range:`bytes=${range}`},(url,options)=>fetch(url,{...options,signal:AbortSignal.any([options.signal,signal])}));content=await readPrintBundleRangeV1(response,file,bundle.archiveBytes,sink.control);}', 'const fleet=sharedFleetFetchContextV1(context.workspace),known=await fleetBlobBytesV1(fleet,file.sha256);if(known&&known.byteLength===file.bytes&&hash(known)===file.sha256)content=known;else{const range=`${file.offset}-${file.offset+file.bytes-1}`,response=await fetchPrintHttpsV1(bundle.url,sink.control,{Range:`bytes=${range}`},(url,options)=>fetch(url,{...options,signal:AbortSignal.any([options.signal,signal])}));content=await readPrintBundleRangeV1(response,file,bundle.archiveBytes,sink.control);await fleetBlobPublishBytesV1(fleet,content);}}')
for item in log:
    print(item[0][-50:], item[1][:8], "->", item[2][:8])
