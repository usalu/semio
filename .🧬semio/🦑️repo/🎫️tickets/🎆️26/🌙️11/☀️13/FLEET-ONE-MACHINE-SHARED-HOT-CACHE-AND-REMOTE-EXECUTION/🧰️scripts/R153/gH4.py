import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from stagelib import *
g=Group("H4")
p=L[:L.rindex("/")]+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts"
g.sub(p,'nativeImageBackingFilenameV1:contract.nativeImageBackingFilenameV1,','nativeStorageBackendOfDirectoryV1:()=>({backingFilename:path.basename(image)}),nativePhysicalIdentityTokenV1:(await import(new URL("../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts",import.meta.url).href)).nativePhysicalIdentityTokenV1,nativeImageBackingAllocatedBytesV1:(row:{size:number|bigint})=>row.size,nativeImageBackingFilenameV1:contract.nativeImageBackingFilenameV1,')
g.sub(p,'names=["protectedMaintenancePublicationSnapshotV1","protectedMaintenancePhysicalFactsV1","captureNativeProtectedMaintenancePhysicalV1"],functions=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??""));expect(functions).toHaveLength(3);const root=fs.mkdtempSync(path.join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pp-"))','names=["protectedMaintenancePublicationSnapshotV1","protectedMaintenancePhysicalFactsV1","captureNativeProtectedMaintenancePhysicalV1","fixedBackingIdentityV1"],functions=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??""));expect(functions).toHaveLength(4);const root=fs.mkdtempSync(path.join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pp-"))')
g.sub(p,'if(name.includes("🧹️cleanup/🎛️command/"))return{...actual,capturePersistentNativeProtectedMaintenanceV1:provider};if(name.includes("🧱️fixed/"))return{...actual,captureFixedNativeProtectedMaintenanceV1:provider,retireNativeFixedPlatformWithWriterV1:canonical};if(name==="../../🍎️image/🟦️.ts")return{...actual,retireNativeImageStorageWithWriterV1:canonical};return actual;','if(name.includes("🔌️backends/📋️contract/"))return{...actual,nativeStorageBackendOfDirectoryV1:(directory:string)=>({...actual.nativeStorageBackendOfDirectoryV1(directory),id:"nested-source"}),nativeStorageOperationsV1:()=>({captureProtectedMaintenance:provider,retireWithWriter:canonical})};return actual;')
import re
t=g.text(p)
pat=re.compile(r'if\(name\.includes\("🧹️cleanup/🎛️command/"\)\)return\{\.\.\.actual,capturePersistentNativeProtectedMaintenanceV1:provider\};if\(name\.includes\("🧱️fixed/"\)(?:&&name\.endsWith\("🟦️\.ts"\))?\)return\{\.\.\.actual,captureFixedNativeProtectedMaintenanceV1:provider(,retireNativeFixedPlatformWithWriterV1:canonical)?\};(?:if\(name==="\.\./\.\./🍎️image/🟦️\.ts"\)return\{\.\.\.actual,retireNativeImageStorageWithWriterV1:canonical\};)?')
def rep(m): return 'if(name.includes("🔌️backends/📋️contract/"))return{...actual,nativeStorageBackendOfDirectoryV1:(directory:string)=>({...actual.nativeStorageBackendOfDirectoryV1(directory),id:"nested-source"}),nativeStorageOperationsV1:()=>({captureProtectedMaintenance:provider'+(',retireWithWriter:canonical' if m.group(1) else '')+'})};'
t2,n=pat.subn(rep,t);print("block sites",n)
tern='return name.includes("🧹️cleanup/🎛️command/")?{...actual,capturePersistentNativeProtectedMaintenanceV1:provider}:name.includes("🧱️fixed/")?{...actual,captureFixedNativeProtectedMaintenanceV1:provider}:actual;'
newt='return name.includes("🔌️backends/📋️contract/")?{...actual,nativeStorageBackendOfDirectoryV1:(directory:string)=>({...actual.nativeStorageBackendOfDirectoryV1(directory),id:"nested-source"}),nativeStorageOperationsV1:()=>({captureProtectedMaintenance:provider})}:actual;'
print("ternary sites",t2.count(tern))
g2=t2.replace(tern,newt)
g_=g2
g.files[p][0]=g_
g.save()
