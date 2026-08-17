import {
  policyExtractGraphqlSchemaFields,
  policyExtractProtobufSchemaFields,
} from "../../../../../../📜️script.ts";

const proto = `syntax = "proto3";
message WiresArtifact {
  // @state persistent
  DslValue wires_fixture = 1;
}
message DslValue {}`;
const gql = `type WiresArtifact {
  wiresFixture: DslValue! @state(class: PERSISTENT)
}
scalar DslValue`;
console.log("proto", policyExtractProtobufSchemaFields(proto).fields[0]);
console.log("gql", policyExtractGraphqlSchemaFields(gql).fields[0]);
