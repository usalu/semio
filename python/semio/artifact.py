from typing import Any, Tuple, Union
from numbers import Number
from dataclasses import dataclass, field
from asyncio import gather
from importlib import resources as impresources
from . import schema
from rdflib import Graph, URIRef, Namespace, Literal
from rdflib.namespace import RDF
from pyshacl import validate as psvalidate

semioAsset = (impresources.files(assets) / 'semio.ttl')
with semioAsset.open("rt", encoding="utf-8") as f:
    semio = f.read()
SE = Namespace("http://github.com/usalu/semio/schema/linkeddata/semio.ttl#")
SESH = Graph().parse(semio)

@dataclass(slots=True)
class Artifact():
    _kind:str = ""
    _graph:Graph = field(default_factory=Graph)
    _uri:URIRef = field(default_factory=URIRef)

    @staticmethod
    async def Parse(kind:str, graph:Union[Graph,str])->Artifact:
        """Parse the artifact from a graph. This assumes there is only node of the kind which can be selected as main uri.
        On succes, the artifact is returned. Otherwise an ArtifactParsingError will be thrown."""
        if isinstance(graph,str):
            graph = Graph().parse(graph)
        uri = graph.value((None,RDF.type,SE[kind]))
        artifact = Artifact(kind,graph,uri)
        minOneShape = (
            f""
            f""
        )

        (conforms, validationGraph, text) = await artifact.validate(minOneShape)
        

    async def validate(self, shapes = None):
        """Perform validation on artifact."""

        if shapes:
            if not isinstance(shapes,list):
                shapes=[shapes]
        shapes.append(SESH)
        for i, shape in enumerate(shapes):
            if isinstance(shapes,str):
                shape=Graph().parse(shapes)
            elif not isinstance(shapes,Graph):
                raise ValueError(f"Shape at position: {i} is not a valid RDF graph.")
            conformsShape, validationGraphShape, textShape = psvalidate(self._graph,shacl_graph=shape)
            conforms &= conforms
            validationGraph += validationGraphShape
            text = text + "\n" + textShape

        return (conforms, validationGraph, text)
    
    def __getattr__(self, name: str) -> Any:
        return self._graph.value(self._uri,SE[name])
    
    def __setattr__(self, name: str, value: Any) -> Any:
        if name in Artifact.__slots__:
            object.__setattr__(self, name, value)
        # Check if value is an Artifact or a Literal
        elif isinstance(value,str) or isinstance(value,Number):
            self._graph.set((self._uri,SE[name],Literal(value)))
        elif isinstance(value, Artifact):
            self._graph += value._graph
            self._graph.set((self._uri,SE[name],value._uri))
        else:
            raise ValueError(f"The value of {name} needs to be either an Artifact or a Literal like a string or a number.")
    
CT = Namespace("http://github.com/usalu/semio/examples/nakagincapsuletower/repository.ttl#")
g = Graph()
g.add((CT.lengthnvp,SE.name,Literal("Length")))
g.add((CT.lengthnvp,SE.value,Literal(4.2)))
g.add((CT.capsulegne,SE.namevaluepair,CT.lengthnvp))
a = Artifact("Gene",g , CT.capsulegne)
lengthnvp = a.namevaluepair

a.description = "This is a capsule gene"
t = a.dere 

print(a.description)

g2 = Graph()
g2.add((CT.widthnvp,SE.name,Literal("Width")))
g2.add((CT.widthnvp,SE.value,Literal(2.7)))
nvp = Artifact("Namevaluepair",g2 , CT.capsulegne)
a.namevaluepair = nvp

print(lengthnvp)

