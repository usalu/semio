
use super::*;

#[semio_framework_async_macros::async_test]
async fn playbook_io_declares_the_extra_chapters_in_port() {
    let io = playbook_io();
    let ports = io.all_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in port declared");
    assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
    assert_eq!(chapters_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    assert_eq!(chapters_in.direction, semio_framework_plugin::MediaPortDirection::In);
}
