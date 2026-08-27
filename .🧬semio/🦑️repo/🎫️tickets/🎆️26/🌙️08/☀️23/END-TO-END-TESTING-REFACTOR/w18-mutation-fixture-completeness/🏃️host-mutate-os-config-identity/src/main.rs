#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/mutate-os-config-identity/🦀️component.rs"]
mod adapter;

fn main() -> std::process::ExitCode {
    semio_repo_test_host::run_main(adapter::adapter())
}
