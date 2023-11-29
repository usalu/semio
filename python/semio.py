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

from typing import Optional, List
from dataclasses import dataclass, field
from argparse import ArgumentParser
from sqlalchemy import TEXT, BLOB, REAL, INTEGER, ForeignKey, create_engine
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship


class Base(DeclarativeBase):
    pass


class Formation(Base):
    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(TEXT(100))
    explanation: Mapped[Optional[str]] = mapped_column(TEXT(3000))
    parts: Mapped[List["Part"]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[List["Attraction"]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Formation(id={self.id!r}, name={self.name!r})"


class Part(Base):
    __tablename__ = "part"

    id: Mapped[int] = mapped_column(primary_key=True)
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped[Formation] = relationship("Formation", back_populates="parts")
    attractings: Mapped[List["Attraction"]] = relationship(
        "Attraction", back_populates="attracting_part"
    )
    attracteds: Mapped[List["Attraction"]] = relationship(
        "Attraction", back_populates="attracted_part"
    )

    def __repr__(self) -> str:
        return f"Part(id={self.id!r}, formation_id={self.formation_id!r})"


class Attraction(Base):
    __tablename__ = "attraction"

    formation_id: Mapped[int] = mapped_column(
        ForeignKey("formation.id"), primary_key=True
    )
    formation: Mapped[Formation] = relationship(
        "Formation", back_populates="attractions"
    )

    attracting_part_id: Mapped[int] = mapped_column(
        ForeignKey("part.id"), primary_key=True
    )
    attracting_part: Mapped[Part] = relationship("Part", back_populates="attractings")

    attracted_part_id: Mapped[int] = mapped_column(
        ForeignKey("part.id"), primary_key=True
    )
    attracted_part: Mapped[Part] = relationship("Part", back_populates="attracteds")

    def __repr__(self) -> str:
        return f"Part(id={self.id!r}, formation_id={self.formation_id!r})"


# class Archive:
#     name: str
#     explanation: str = ""
#     types: Dict[str, Type] = field(default_factory=dict)
#     formations: Dict[str, Formation] = field(default_factory=dict)


if __name__ == "__main__":
    # parser = ArgumentParser(description="Process some integers.")
    # parser.add_argument("--argument", metavar="N", type=str)
    # args = parser.parse_args()
    # main.run(args)
    engine = create_engine("sqlite:///metabolism.db", echo=True)
    Base.metadata.create_all(engine)
