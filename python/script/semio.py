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

from typing import Union
from abc import ABC, abstractmethod
from pydantic import BaseModel
from fastapi import FastAPI

class Representation(BaseModel):
    url: str
    lod: str
    tags: list[str]


class Specifier(BaseModel):
    context: str
    group: str


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
    specifiers: list[Specifier]


class PortId(BaseModel):
    specifiers: list[Specifier]


class Quality(BaseModel):
    name: str
    value: str
    unit: str


class Type(BaseModel):
    name: str
    explanation: str
    icon: str
    representations: list[Representation]
    ports: list[Port]
    qualities: list[Quality]


class TypeId(BaseModel):
    name: str
    qualities: list[Quality]


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


class Formation(BaseModel):
    name: str
    explanation: str
    icon: str
    pieces: list[Piece]
    attractions: list[Attraction]
    qualities: list[Quality]


class FormationId(BaseModel):
    name: str
    qualities: list[Quality]


class Kit(BaseModel):
    name: str
    explanation: str
    icon: str
    url: str
    types: list[Type]
    formations: list[Formation]


class Parameter(BaseModel):
    name: str
    value: str


class Script(ABC,BaseModel):
    name: str
    explanation: str
    icon: str
    parameters: list[Parameter]


class Prototype(Script):
    pass

class Modification(Script):
    pass

class Choreography(Script):
    pass


class Hoster(FastAPI):
    def __init__(self,):
        super().__init__()

hoster = Hoster()

@hoster.get("/")
def read_root():
    return {"scripts":"A description of the scripts available."}


@hoster.post("/transform/{name}")
def transformations(name:str ,formation: Formation, parameters: list[Parameter]) -> Formation:
    return {"formation": formation}