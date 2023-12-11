#!/usr/bin/env python

# semio
# Copyright (C) 2023 Ueli Saluz

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

"""
One file executable with all functionality around semio.
"""
from sys import stdin, stdout, stderr
from typing import Optional, List
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from argparse import ArgumentParser, FileType
from sqlalchemy import TEXT, BLOB, REAL, INTEGER, ForeignKey, create_engine
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship

# Persistance layer
# -----------------
# The persistance layer is designed that all artifacts (except blob properties)
# can be stored/merged/ in a rdf-store that supports named graphs.
# As all artifacts should be disk-compatible and deleting related named graphs
# is not trivial, technically the peristance layer consits of two layers.
# The first layer is the container layer and the second layer is the data layer.
# The container layer is a sqlite database that keeps track of high level hierarchical data.
# It cascade deletes all children when a parent is deleted. The data layer
# stores the actual data in rdf. As serialization format we use canonicalized
# nquads with URDNA 2015 normalization.


class Base(DeclarativeBase):
    pass


class Archive(Base):
    __tablename__ = "archive"

    sha3_224: Mapped[bytes] = mapped_column(BLOB(224), primary_key=True)
    # boolean column that uses 0 and 1 as values
    is_root: Mapped[bool] = mapped_column(INTEGER())
    nquads_urdna_15: Mapped[str] = mapped_column(TEXT())
    formations: Mapped[List["Formation"]] = relationship(
        "Formation", back_populates="archive"
    )
    types: Mapped[List["Type"]] = relationship("Type", back_populates="archive")

    def __repr__(self) -> str:
        return f"Archive(sha3_224={self.sha3_224!r})"


class Formation(Base):
    __tablename__ = "formation"

    sha3_224: Mapped[bytes] = mapped_column(BLOB(224), primary_key=True)
    archive_sha3_224: Mapped[bytes] = mapped_column(ForeignKey("archive.sha3_224"))
    archive: Mapped[Archive] = relationship("Archive", back_populates="formations")
    nquads_urdna_15: Mapped[str] = mapped_column(TEXT())

    def __repr__(self) -> str:
        return f"Formation(sha3_224={self.sha3_224!r}, archive_sha3_224={self.archive_sha3_224!r})"


class Type(Base):
    __tablename__ = "type"

    sha3_224: Mapped[bytes] = mapped_column(BLOB(224), primary_key=True)
    nquads_urdna_15: Mapped[str] = mapped_column(TEXT())
    archive_sha3_224: Mapped[bytes] = mapped_column(ForeignKey("archive.sha3_224"))
    archive: Mapped[Archive] = relationship("Archive", back_populates="formations")
    blob_properties: Mapped[List["BlobProperty"]] = relationship(
        "BlobProperty", back_populates="type"
    )

    def __repr__(self) -> str:
        return f"Type(sha3_224={self.sha3_224!r}, archive_sha3_224={self.archive_sha3_224!r})"


class BlobProperty(Base):
    __tablename__ = "blob_property"

    sha3_224: Mapped[bytes] = mapped_column(BLOB(224), primary_key=True)
    type_sha3_224: Mapped[bytes] = mapped_column(ForeignKey("type.sha3_224"))
    type: Mapped[Type] = relationship("Type", back_populates="blob_properties")
    nquads_urdna_15: Mapped[str] = mapped_column(TEXT())
    blob: Mapped[bytes] = mapped_column(BLOB())

    def __repr__(self) -> str:
        return f"BlobProperty(sha3_224={self.sha3_224!r}, archive_sha3_224={self.archive_sha3_224!r})"


# class Command(ABC):
#     @abstractmethod
#     async def execute(self):
#         pass


# class RawCommand(Command):
#     def __init__(self, rawCommand: str):
#         self.rawCommand = rawCommand


# class ArchiveCommand(Command, ABC):
#     pass


# class InitializeArchiveCommand(ArchiveCommand):
#     write = True
#     pass


# class AddToArchiveCommand(ArchiveCommand):
#     write = True
#     pass


semioParser = ArgumentParser(
    description=(
        "Parameterless command line interface which only operates with stdin, stdout and stderr."
    )
)
semioParser.add_argument(
    "--version",
    action="version",
    version="2.0.0",
    help="Print version and exit.",
)
semioParser.add_argument("input", nargs="?", type=FileType("r"), default=stdin)
semioParser.add_argument("output", nargs="?", type=FileType("w"), default=stdout)

if __name__ == "__main__":
    semioParser.parse_args()
