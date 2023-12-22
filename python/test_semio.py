from graphene.test import Client

from semio import schema

createLocalKitFromBaseMutation = """mutation CreateLocalKitFromBase($directory: String!, $base: KitBaseInput! ){
  createLocalKit(
    directory:$directory,
    kitInput:{
          base:$base
    }
  ){
    ... on KitNode{
      uri
      name
      explanation
      symbol
    }
    ... on CreateLocalKitErrorNode{
      error
    }
  }
}"""

deleteLocalKitMutation = """mutation DeleteLocalKit($directory: String!){
  deleteLocalKit(
    directory:$directory,
  ){
    error
  }
}"""


def test_kit_crud(tmp_path):
    client = Client(schema)
    name = "metabolistic"
    explanation = "A metabolism clone"
    symbol = "🫀"
    uri = "https://github.com/usalu/semio/tree/1.x/examples/metabolistic"
    kitBase = {
        "base": {
            "uri": uri,
            "characterization": {
                "name": name,
                "explanation": explanation,
                "symbol": symbol,
            },
        }
    }
    executed = client.execute(
        createLocalKitFromBaseMutation,
        context_value={"directory": tmp_path, "base": kitBase},
    )
    assert executed == {
        "data": {
            "createLocalKit": {
                "uri": uri,
                "name": name,
                "explanation": explanation,
                "symbol": symbol,
            }
        }
    }
