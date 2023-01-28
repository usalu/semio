from typing import Iterable
from semio.model import Sobject,Assembly,Layout,Element, LAYOUTSTRATEGY_BREADTHFIRST
from semio.backend.assembler import Assembler

from google.protobuf.json_format import ParseDict
from json import load

from os.path import realpath,dirname,join

from pytest import fixture,mark

@fixture
def assembler():
    return Assembler()

@mark.parametrize("layoutJsonPath, assembliesJsonPath",
[
    ("requests/assembler/sample_layoutDesignRequest_02.json","requests/assembler/sample_02_result.json")
])
def test_assembler_assemblyToElements(layoutJsonPath,assembliesJsonPath,assembler):
    path_to_current_file = realpath(__file__)
    current_directory = dirname(path_to_current_file)
    path_to_file = join(current_directory, layoutJsonPath)
    with open(path_to_file, 'r') as file:
        sample = load(file)
        layout = ParseDict(sample, Layout())
        result = assembler.assemblyToElements(layout)
    path_to_solution_file = join(current_directory, assembliesJsonPath)
    with open(path_to_solution_file) as file:
        correct_result = [ParseDict(assembly, Assembly()) for assembly in load(file)["assemblies"]]
    assert result == correct_result

def assemblyToElements(self: Assembler, sobjects:Iterable[Sobject],assembly:Iterable[Assembly])->Iterable[Element]:
    # element : Element = self.RequestElement(
        # sobject: Sobject = Sobject(),
        # target_representation_platforms:Iterable[Platform] = [],
        # target_representation_concepts:Iterable[str] = [],
        # target_representation_lods:Iterable[int] = [],
        # targets_required:bool = False)
    # pose : Pose, point: Point = self.ConnectElement(
        # sobjects:Tuple[Sobject,Sobject],
        # connection:Connection))
    raise NotImplementedError()

#test_get_assemblies_from_layoutrequest()
#visualize_append_problem()