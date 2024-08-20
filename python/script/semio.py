#!/usr/bin/env python

# semio script.
# Copyright (C) 2024 Ueli Saluz

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

"""
semio script.
"""

from typing import Union, List
from abc import ABC, abstractmethod
from pydantic import BaseModel
from fastapi import FastAPI
from uvicorn import run

SEMIO_HOST_HOST = "0.0.0.0"
SEMIO_HOST_PORT = 8000


class Representation(BaseModel):
    url: str
    lod: str
    tags: List[str]


class Locator(BaseModel):
    group: str
    subgroup: str


class Point(BaseModel):
    x: float
    y: float
    z: float


class Vector(BaseModel):
    x: float
    y: float
    z: float


class Plane(BaseModel):
    origin: Point
    x_axis: Vector
    y_axis: Vector


class Port(BaseModel):
    plane: Plane
    locators: List[Locator]


class PortId(BaseModel):
    locators: List[Locator]


class Quality(BaseModel):
    name: str
    value: str
    unit: str


class Type(BaseModel):
    name: str
    description: str
    icon: str
    representations: List[Representation]
    ports: List[Port]
    qualities: List[Quality]


class TypeId(BaseModel):
    name: str
    qualities: List[Quality]


class Piece(BaseModel):
    id: str
    type: TypeId


class TypePieceSide(BaseModel):
    port: PortId


class PieceSide(BaseModel):
    id: str
    type: TypePieceSide


class Side(BaseModel):
    piece: PieceSide


class Attraction(BaseModel):
    attracting: Side
    attracted: Side


class Design(BaseModel):
    name: str
    description: str
    icon: str
    pieces: List[Piece]
    attractions: List[Attraction]
    qualities: List[Quality]


class DesignId(BaseModel):
    name: str
    qualities: List[Quality]


class Kit(BaseModel):
    name: str
    description: str
    icon: str
    url: str
    types: List[Type]
    designs: List[Design]


class Parameter(BaseModel):
    name: str
    value: str


class Script(ABC):
    name: str
    description: str
    icon: str
    parameters: List[Parameter]


class Prototype(Script, ABC):
    @abstractmethod
    def prototype(self, parameters: List[Parameter]) -> Type:
        pass


class Modification(Script, ABC):
    @staticmethod
    @abstractmethod
    def modify(type: Type, parameters: List[Parameter]) -> Type:
        pass


class Choreography(Script, ABC):
    @staticmethod
    @abstractmethod
    def choreograph(parameters: List[Parameter]) -> Design:
        pass


class Transdesign(Script, ABC):
    @staticmethod
    @abstractmethod
    def transform(design: Design, parameters: List[Parameter]) -> Design:
        pass


class Synthesis(Script, ABC):
    @staticmethod
    @abstractmethod
    def synthesize(design: Design, parameters: List[Parameter]) -> Quality:
        pass


class Host(FastAPI):
    def __init__(
        self,
        prototypes: List[Prototype] = [],
        modifications: List[Modification] = [],
        choreographies: List[Choreography] = [],
        transformations: List[Transdesign] = [],
        syntheses: List[Synthesis] = [],
    ):
        super().__init__(separate_input_output_schemas=False)

        self.add_api_route("/", self.read_root, methods=["GET"])
        self.add_api_route("/transform/{name}", self.transform, methods=["POST"])

        self.prototypes = prototypes
        self.modifications = modifications
        self.choreographies = choreographies
        self.transformations = transformations
        self.syntheses = syntheses

    def read_root(self):
        return {"scripts": "A description of the scripts available."}

    def transform(
        self, name: str, design: Design, parameters: List[Parameter]
    ) -> Design:
        design = [p for p in self.transformations if p.name == name][0].transform(
            design, parameters
        )
        return {"design": design}

    def run(self):
        run(self, host=SEMIO_HOST_HOST, port=SEMIO_HOST_PORT)
