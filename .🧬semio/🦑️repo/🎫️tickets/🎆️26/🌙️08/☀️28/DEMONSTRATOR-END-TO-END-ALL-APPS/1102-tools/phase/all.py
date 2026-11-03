import importlib,sys,os,shutil,subprocess,glob
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import ph
for m in "s_arith s_derive s_draw s_ordered_plan s_small1 s_small2 s_pack s_misc1 s_text s_job s_os1 s_derived s_checkout s_hist s_wgpu s_plugin s_fem s_persist".split():
    importlib.import_module(m)
ph.build()
ORDER="""semio-framework-value-control-contract semio-framework-value-allocation-return semio-framework-value-native-owned semio-framework-value-persistent semio-framework-value-derive-expansion semio-framework-value-derive semio-framework-value-edit semio-framework-value-paged-list semio-framework-value-ordered semio-framework-value-clone-source semio-framework-value-retained-map semio-framework-pack-record-retained semio-framework-actor-retained-turn semio-framework-replication-history semio-framework-text-splice-native semio-framework-job semio-framework-job-failure semio-framework-job-payload semio-framework-os-flow-wasm semio-framework-os-flow-extensions semio-framework-os-store-initialization-replay semio-framework-os-store-replay-operation semio-framework-os-store-engine-replay semio-framework-os-store-document semio-framework-os-history semio-framework-os-renderer-wgpu semio-framework-plugin semio-framework-plugin-job semio-framework-plugin-job-receiving semio-framework-plugin-tool-run-turns semio-framework-plugin-decl-emission semio-framework-plugin-decl-app semio-framework-plugin-window-config semio-s-artifact-draw-drawing-scene semio-s-artifact-fem-3d-editor-results-window""".split()
assert set(ORDER)==set(ph.SP),(set(ph.SP)^set(ORDER))
open(ph.OUT+"/order.txt","w").write("\n".join(ORDER)+"\n")
open(ph.OUT+"/supersedes.txt","w").write("\n".join(f"{k} {os.path.basename(v)}" for k,v in ph.SUPERSEDES.items())+"\n")
print("superseded policy patches:",list(ph.SUPERSEDES))
