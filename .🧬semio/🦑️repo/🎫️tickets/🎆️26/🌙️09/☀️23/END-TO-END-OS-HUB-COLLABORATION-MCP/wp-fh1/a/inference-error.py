"""💡️ FH1 family A — `ArtifactInferenceExecutionError.code` is a `FaultCode`: every inference refusal names its literal
code where it is raised (framework, hub and the plugins' inference services), so the fault census reads it, and the
bounded infer job hands the code on unchanged."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex
P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
apply("", [
    (P, '''    pub struct ArtifactInferenceExecutionError {
        pub code: &'static str,
        pub message: String,
    }''', '''    pub struct ArtifactInferenceExecutionError {
        pub code: FaultCode,
        pub message: String,
    }'''),
    (P, '''    impl ArtifactInferenceExecutionError {
        pub fn new(code: &'static str, message: impl Into<String>) -> Self {''', '''    impl ArtifactInferenceExecutionError {
        pub fn new(code: FaultCode, message: impl Into<String>) -> Self {'''),
    (P, '''    impl std::fmt::Display for ArtifactInferenceExecutionError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "{}: {}", self.code, self.message)''', '''    impl std::fmt::Display for ArtifactInferenceExecutionError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "{}: {}", self.code.as_str(), self.message)'''),
])
regex("", P, r'ArtifactInferenceExecutionError::new\("([^"]+)"', r'ArtifactInferenceExecutionError::new(FaultCode::new("\1")', 35)
for path, count in [("🌎️hub/💡️inference/🏃️runtime/🦀️.rs", 2), ("🌎️hub/💡️inference/📇️catalog/🧪️tests/🔬️unit/🦀️.rs", 1), ("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs", 10), ("✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs", 1), ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs", 3)]:
    regex("", path, r'ArtifactInferenceExecutionError::new\("([^"]+)"', r'ArtifactInferenceExecutionError::new(semio_framework_plugin::FaultCode::new("\1")', count)
apply("", [("✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/📇️native-codecs/🦀️.rs", 'semio_framework_plugin::ArtifactInferenceExecutionError::new(error, "request stopped")', 'semio_framework_plugin::ArtifactInferenceExecutionError::new(semio_framework_plugin::FaultCode::new(error), "request stopped")')])
