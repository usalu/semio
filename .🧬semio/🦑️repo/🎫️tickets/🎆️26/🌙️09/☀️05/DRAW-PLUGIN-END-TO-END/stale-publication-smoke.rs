use semio_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_cancellation_publication_boundaries;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestApp;

fn main() {
    semio_framework_async::block_on(test_retained_cancellation_publication_boundaries::<TestApp<false, 1>>());
}
