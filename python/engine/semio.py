from functools import lru_cache
from pathlib import Path
from typing import List, Optional
from urllib.parse import urlparse
from sqlmodel import (
    SQLModel,
    Field as SQLField,
    Relationship,
)
from sqlalchemy import (
    CheckConstraint,
    UniqueConstraint,
    String as SQLString,
    create_engine,
)
from sqlalchemy.orm import sessionmaker, Session, validates

NAME_LENGTH_MAX = 100


@lru_cache(maxsize=100)
def getLocalSession(path: str) -> Session:
    engine = create_engine("sqlite:///" + path)
    SQLModel.metadata.create_all(engine)
    return sessionmaker(bind=engine)()


class TagModel(SQLModel, table=True):
    """🏷️ A tag is meta-data for grouping representations."""

    __tablename__ = "tag"

    value: str = SQLField(
        primary_key=True,
    )
    representationId: int = SQLField(
        foreign_key=("representation.id"), primary_key=True
    )
    representation: "RepresentationModel" = Relationship(back_populates="_tags")


class RepresentationModel(SQLModel, table=True):
    """💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime."""

    __tablename__ = "representation"
    id: Optional[int] = SQLField(default=None, primary_key=True, exclude=True)
    localId: str = SQLField(alias="lid")
    url: str
    _tags: List[TagModel] = Relationship(
        back_populates="representation", cascade_delete=True
    )
    typeId: Optional[int] = SQLField(default=None, foreign_key=("type.id"))
    type: "TypeModel" = Relationship(back_populates="representations")
    __table_args__ = (UniqueConstraint("url"),)

    @property
    def tags(self) -> List[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: List[str]):
        self._tags = [TagModel(value=tag) for tag in tags]

    @validates("url")
    def validate_url(self, key: str, url: str):
        return url


class TypeModel(SQLModel, table=True):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    __tablename__ = "type"

    id: Optional[int] = SQLField(default=None, primary_key=True)
    representations: List[RepresentationModel] = Relationship(
        back_populates="type", cascade_delete=True
    )

    # __table_args__ = (UniqueConstraint("name", "variant", "kitId"),)

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name}, description={self.description}, icon={self.icon}, variant={self.variant} unit={self.unit}, kitId={self.kitId!r}, representations={self.representations!r}, ports={self.ports!r}, qualities={self.qualities!r}, pieces={self.pieces!r})"

    def __str__(self) -> str:
        return f"Type(id={str(self.id)}, kitId={str(self.kitId)})"

    def client__str__(self) -> str:
        return f"Type(name={self.name}, variant={self.variant})"


def create_db_and_tables():
    r1 = RepresentationModel(localId="g", url="https://www.google.com")
    r2 = RepresentationModel(localId="y", url="https://www.yahoo.com")
    r2.tags = ["tag1", "tag2"]
    r3 = RepresentationModel(localId="y2", url="https://www.yahoo.com1")
    t1 = TypeModel()
    t1.representations = [r1, r2, r3]
    engine = create_engine("sqlite:///test.sqlite3")
    SQLModel.metadata.create_all(engine)
    with Session(engine) as session:
        session.add(t1)
        session.commit()
        pass


# create_db_and_tables()
