import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from patchlib import rep, sha

P = sys.argv[1]
print(P[-30:], sha(P))

rep(P, ' if(existsSync(cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt))try{return await preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"shared");}catch(error){if(!(error instanceof CargoPreparationExclusiveV1))throw error;}\n return preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"exclusive");',
 ' let commit:CargoPreparationCommitV1|undefined;\n if(!existsSync(cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt+recipeMarkerV1))try{return await preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"shared");}catch(error){if(!(error instanceof CargoPreparationExclusiveV1))throw error;commit=error.commit;}\n return preparedPair(storage,root,manifest,control,runner,identity,preparation,sourcePaths,packages,toolInputs,"exclusive",commit);')

rep(P, '/** 🔗️ Replans pre-command manifest edits, then owns update and locked fetch under original exclusive custody; a receipt that is still current is confirmed under a shared lease so concurrent runs overlap, and only a pass that must run a recipe, write a membership or renew the receipt takes the exclusive lease, and leaves a witness of the held receipt for launchers when the identity names its tool files.',
 '/** 🔗️ Replans pre-command manifest edits, then owns update and locked fetch under original exclusive custody. Every run plans, hashes and confirms its receipt under the shared lease, so concurrent runs overlap; only a pass that must run a recipe, write a membership or renew the receipt takes the exclusive lease, and a renewal does not plan again there: it re-verifies the inputs it planned, finds the receipt a peer renewed meanwhile or runs update and locked fetch alone, so the exclusive section is the mutation, not the planning; a selection known to run recipes goes straight to the exclusive lease. A launcher gets a witness of the held receipt when the identity names its tool files.')

rep(P, '/** 🚧️ Signals that a shared pass found work only the exclusive holder may do: a recipe to run, a membership to write, or a receipt to renew. */\nclass CargoPreparationExclusiveV1 extends Error{}',
 '/** 🏷️ Names the hint beside a receipt that the last exclusive pass of its selection ran recipes. */\nconst recipeMarkerV1=".recipes";\n/** 🏷️ Remembers beside the receipt whether the last exclusive pass of a selection ran recipes, so the next run goes straight to the exclusive lease instead of running them under the shared lease first; a hint that cannot be written only costs one shared pass. */\nfunction markCargoPreparationRecipesV1(receiptPath:string,recipes:boolean):void{try{if(recipes)writeFileSync(receiptPath+recipeMarkerV1,"1");else rmSync(receiptPath+recipeMarkerV1,{force:true});}catch{}}\n/** 🪜️ Finishes a planned renewal under the exclusive lease in the directory of that pass: it verifies the planned inputs still hold (else it throws the replan signal and the exclusive pass plans again), accepts a receipt a peer wrote meanwhile or runs update and locked fetch. */\ntype CargoPreparationCommitV1=(directory:string)=>Promise<void>;\n/** 🚧️ Signals that a shared pass found work only the exclusive holder may do: a recipe to run, a membership to write, or a receipt to renew (then it carries the planned renewal). */\nclass CargoPreparationExclusiveV1 extends Error{readonly commit?:CargoPreparationCommitV1;constructor(commit?:CargoPreparationCommitV1){super("Cargo preparation needs the exclusive lease");this.commit=commit;}}')

rep(P, 'receiptPath=cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt;\n', 'receiptPath=cargoPreparationRecordPathsV1(storage,root,manifest,packages).receipt;\n if(mode==="exclusive")markCargoPreparationRecipesV1(receiptPath,prepared.recipes.length>0);\n')

rep(P, 'toolInputs:readonly string[],mode:"shared"|"exclusive"):Promise<void> {', 'toolInputs:readonly string[],mode:"shared"|"exclusive",commit?:CargoPreparationCommitV1):Promise<void> {')

rep(P, ' mkdirSync(directory,{recursive:true});const sources=', ' mkdirSync(directory,{recursive:true});if(commit){try{await commit(directory);return;}catch(error){if(!(error instanceof CargoPreparationManifestChangedV1))throw error;}}\n const sources=')

rep(P, ' const lockBefore=cargoPreparationInputV1(root,resolve(root,scope.lock),"file"),witness=(lockSha256:string):void=>{if(toolInputs.length&&!prepared.recipes.length)writeCargoPreparationWitnessV1(storage,root,manifest,packages,directory,',
 ' const settle=async(directory:string,exclusive:boolean):Promise<void>=>{\n const lockBefore=cargoPreparationInputV1(root,resolve(root,scope.lock),"file"),witness=(lockSha256:string):void=>{if(toolInputs.length&&!prepared.recipes.length)writeCargoPreparationWitnessV1(storage,root,manifest,packages,directory,')

rep(P, ' if(mode==="shared")throw new CargoPreparationExclusiveV1();\n await runner(',
 ' if(!exclusive)throw new CargoPreparationExclusiveV1(async next=>{await current();if(await identity(root,signal)!==tool)throw new CargoPreparationManifestChangedV1("Prepared tool changed");await settle(next,true);});\n await runner(')

rep(P, 'renameSync(temporary,receiptPath);witness(lock.sha256);\n return;\n }catch(error){',
 'renameSync(temporary,receiptPath);witness(lock.sha256);\n };\n await settle(directory,mode==="exclusive");return;\n }catch(error){')
print("cargo preparation patched")
