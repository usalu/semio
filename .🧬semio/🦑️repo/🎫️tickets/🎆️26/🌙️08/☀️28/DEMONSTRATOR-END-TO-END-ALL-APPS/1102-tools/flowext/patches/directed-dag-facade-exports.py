import sys
G="🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/🦀️.rs"
s=open(G).read()
a="    pub use semio_framework_os_infinite_board_dag_graph::{dag_screen_to_world, dag_world_to_screen};\n"
b="    pub use semio_framework_os_infinite_board_dag_host::{dag_take_pending_open_instance_id, DagHost, DagHostRetirement};\n"
assert s.count(a)==1 and s.count(b)==1 and "camera_content_coverage" not in s
s=s.replace(a,a+"    pub use semio_framework_os_infinite_board_dag_graph::camera_content_coverage;\n").replace(b,b+"    pub use semio_framework_os_infinite_board_dag_host::label_overlay_rows_for_node_spec;\n")
open(G,"w").write(s)
