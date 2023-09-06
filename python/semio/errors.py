from dataclasses import dataclass
from rdflib import Graph
from .artifact import Artifact

@dataclass
class InvalidArtifactException(Exception):
    artifact: Artifact

@dataclass
class InvalidKindException(InvalidArtifactException):
    actualKind: str

@dataclass
class InvalidShapeException(InvalidArtifactException):
    validationGraph: Graph

@dataclass
class InvalidSemioShapeException(InvalidShapeException):
    pass

@dataclass
class InvalidCustomShapeException(InvalidShapeException):
    customShape: Graph