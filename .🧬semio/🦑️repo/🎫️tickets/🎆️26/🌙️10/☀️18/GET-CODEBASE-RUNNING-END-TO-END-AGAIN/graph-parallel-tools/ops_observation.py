# 👁️ The observation edits of the graph farm set: sealed records built by a worker, pre-imaged project values, verified block memo. Executed by ops_gen.py in its namespace (edit, span, O, root).

edit(O, 'const image=jsonImage(await compute());if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");return JSON.parse(image);}',
     'const computed=await compute(),image=computed?.[PREIMAGED_PROJECT]??jsonImage(computed);if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");return JSON.parse(image);}')
edit(O, "const result = await inferenceValidationScopes.run(inferenceValidationScopes.getStore()??observations,()=>recordScopes.run({ records, hints }, () => scopes.run(observations, compute)));",
     "const sealing={digest},result = await inferenceValidationScopes.run(inferenceValidationScopes.getStore()??observations,()=>recordScopes.run({ records, hints, sealing }, () => scopes.run(observations, compute)));")
edit(O, "if (jsonImage(result) !== undefined) originalInferencePublications.set(",
     "if (sealing.sealed!==undefined||jsonImage(result) !== undefined) originalInferencePublications.set(")

persist = span(O, "if (consistent && !racy && jsonExact(result) && (nestedProject?", 'usedInference.set(cachePath,createHash("sha256").update(header).digest("hex"));\n  }')
start = persist.index("    const image=JSON.stringify(result)")
tail = "    const packed=await packInferenceRecordV1(Buffer.from(`${header}\\n${image}`)),bytes=packed.byteLength;"
end = persist.index(tail) + len(tail)
sealed_body = '''    const temporary = `${cachePath}.${process.pid}.${started}.tmp`,sealed=sealing.sealed!==undefined&&sealing.sealed.digest===digest&&sealing.sealed.rowCount===rows.length&&persistSealedBlocksV1(dirname(cachePath),sealing.sealed)?sealing.sealed:undefined;
    let header,packed;
    if(sealed===undefined){const image=JSON.stringify(result);header=inferenceHeaderTextV1(digest,createHash("sha256").update(image).digest("hex"),authority,rows,dirname(cachePath));packed=await packInferenceRecordV1(Buffer.from(`${header}\\n${image}`));}else packed=sealed.packed;
    const bytes=packed.byteLength;'''
new_persist = (persist[:start] + sealed_body + persist[end:]).replace("if (consistent && !racy && jsonExact(result) &&", "if (consistent && !racy && (sealing.sealed!==undefined||jsonExact(result)) &&")
new_persist = new_persist.replace('usedInference.set(cachePath,createHash("sha256").update(header).digest("hex"));', 'usedInference.set(cachePath,sealed===undefined?createHash("sha256").update(header).digest("hex"):sealed.headerDigest);')
edit(O, persist, new_persist)

edit(O, "function inferenceHeaderTextV1(digest,resultSha256,authority,rows,directory){",
     "function inferenceHeaderTextV1(digest,resultSha256,authority,rows,directory,persist=persistInferenceBlockV1){")
edit(O, "if(persistInferenceBlockV1(directory,block))blocks.push(block.digest)",
     "if(persist(directory,block))blocks.push(block.digest)")
edit(O, "let known=blocksByIdentity.get(block.key);if(known===undefined){",
     "let known=block.packed===undefined&&block.only!==true?blocksByIdentity.get(block.key):{digest:block.digest};if(known===undefined){")
edit(O, "present=readObservationBlockV1(directory,known.digest,true);if(present&&!present.absent){present.recheck();return true;}if((blocksRefused.get(directory)??0)>Date.now())return false;const packed=brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows)),",
     "present=verifiedBlockV1(path)?{absent:false,recheck:()=>{}}:readObservationBlockV1(directory,known.digest,true);if(present&&!present.absent){present.recheck();return true;}if(block.only===true||(blocksRefused.get(directory)??0)>Date.now())return false;const packed=block.packed??brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows)),")
edit(O, "function readObservationBlockV1(directory,digest,optional=false,budget=observationReadBudgetV1()){const path=inferenceObservationBlockPathV1(directory,digest);const physical=", '''const verifiedBlocks=new Map(),VERIFIED_BLOCK_ROWS=4194304;let verifiedBlockRows=0;
/** 🧊️ Remembers a block whose bytes, hash and rows this process verified, by the full identity of its leaf. */
function rememberVerifiedBlockV1(path,epoch,rows){if(verifiedBlockRows+rows.length>VERIFIED_BLOCK_ROWS){verifiedBlocks.clear();verifiedBlockRows=0;}const prior=verifiedBlocks.get(path);if(prior)verifiedBlockRows-=prior.rows.length;verifiedBlocks.set(path,{epoch,rows,recheck:undefined});verifiedBlockRows+=rows.length;}
/** 🧊️ A verified block while its leaf still has the identity it was verified with (device, inode, mode, links, size and both nanosecond stamps); any other leaf is read and validated again. */
function verifiedBlockV1(path){
 const entry=verifiedBlocks.get(path);if(entry===undefined)return undefined;
 try{const row=fs.lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&row.nlink===1n&&observationEpochV1(row)===entry.epoch){entry.recheck??=()=>{requireRepositoryBuildGraphObservationPathV1(path,"read");const actual=fs.lstatSync(path,{bigint:true});if(!actual.isFile()||actual.isSymbolicLink()||observationEpochV1(actual)!==entry.epoch)throw Error("Observation leaf changed before delivery");};return entry;}}catch{}
 verifiedBlockRows-=entry.rows.length;verifiedBlocks.delete(path);return undefined;
}
function readObservationBlockV1(directory,digest,optional=false,budget=observationReadBudgetV1()){const path=inferenceObservationBlockPathV1(directory,digest),verified=verifiedBlockV1(path);if(verified){requireRepositoryBuildGraphObservationPathV1(path,"read");observationChargeV1(budget,"rows",verified.rows.length);return{rows:verified.rows,recheck:verified.recheck,epoch:verified.epoch};}const physical=''')
edit(O, 'observationChargeV1(budget,"rows",rows.length);physical.recheck();return{rows:internObservationsV1(rows),recheck:physical.recheck};}',
     'observationChargeV1(budget,"rows",rows.length);physical.recheck();const shared=internObservationsV1(rows);rememberVerifiedBlockV1(path,physical.epoch,shared);return{rows:shared,recheck:physical.recheck,epoch:physical.epoch};}')
edit(O, "recheck();return{bytes,recheck};", "recheck();return{bytes,recheck,epoch};")
