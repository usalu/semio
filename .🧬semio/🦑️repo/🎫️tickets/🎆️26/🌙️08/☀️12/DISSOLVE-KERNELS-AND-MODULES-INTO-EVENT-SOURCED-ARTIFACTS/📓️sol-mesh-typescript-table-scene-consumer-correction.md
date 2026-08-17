# Mesh TypeScript Table Scene Consumer Correction

The bounded Mesh audit initially identified only the table window kit as a `TableScene` consumer. A required active-source check before issuing any implementation lease found a second independent production consumer in the protected OS renderer package index.

Current clean fingerprints:

- Mesh TypeScript: `9ec2f6b8c0f569eab8a5c1f755864d3d4986bae618a3473131fb59075cbdf9c8`;
- Table window kit: `358b69888b39990a3fbf4c1b8e6873f06dc309a3e1cac28fbfd4d3f70d522ade`.

Resolved terminals:

1. OS plugin table window kit imports `TableScene` and constructs the table scene sent through `UiComponentSceneNode`.
2. OS renderer imports `TableScene` as its independent rendering boundary.

`TableScene` therefore meets the two-production-component threshold at the framework Mesh contract owner. It is not a one-consumer inline candidate. No Mesh edit is authorized, and the protected renderer remains untouched.
