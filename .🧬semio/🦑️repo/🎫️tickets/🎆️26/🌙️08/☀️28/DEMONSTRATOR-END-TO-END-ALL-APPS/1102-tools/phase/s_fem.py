from ph import *
f="semio-s-artifact-fem-3d-editor-results-window"
R_="📊️results/🦀️.rs"
miss='.ok_or_else(||semio_framework_plugin::PluginAssemblyError::new("fem3d.results-display","result display mode has no eigenvalue"))?'
E(f,R_,'display.mode.display_eigenvalue(eigenvalue).expect("modal display value")','display.mode.display_eigenvalue(eigenvalue)'+miss)
E(f,R_,'display.mode.display_eigenvalue(eigenvalue).expect("buckling display value")','display.mode.display_eigenvalue(eigenvalue)'+miss)
if __name__=="__main__":build()
