// #region Header

// Details.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import { arrayMove } from "@dnd-kit/sortable";
import { Minus, Plus } from "lucide-react";
import { FC } from "react";
import { useTranslation } from "react-i18next";
import { SortableTreeItems, TreeContent, TreeItem } from "../../../../elements/aggregation/Tree";
import { Input } from "../../../../elements/input/Input";
import { Slider } from "../../../../elements/input/Slider";
import Stepper from "../../../../elements/input/Stepper";
import { Textarea } from "../../../../elements/input/Textarea";
import { Author, guid, Guid, Kit, Type } from "../../../../semio";
import { useIsInTypeScope, useKit, useKitCommands, useType } from "../../../kits/store";
import { useTypeAppCommands, useTypeAppHover, useTypeAppSelection } from "../store";

export const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <TypeDetailsForm />;
};

const TypeDetailsForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy label={t("type.name")} value={type.name} onLazyChange={(value) => updateTypeField({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            label={t("type.description")}
            value={type.description || ""}
            placeholder={t("type.descriptionPlaceholder")}
            onLazyChange={(value) => updateTypeField({ description: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.icon")}
            value={type.icon || ""}
            placeholder={t("type.iconPlaceholder")}
            onLazyChange={(value) => updateTypeField({ icon: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.image")}
            value={type.image || ""}
            placeholder={t("type.imagePlaceholder")}
            onLazyChange={(value) => updateTypeField({ image: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            label={t("type.variant")}
            value={type.variant || ""}
            placeholder={t("type.variantPlaceholder")}
            onLazyChange={(value) => updateTypeField({ variant: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input lazy label={t("type.unit")} value={type.unit} onLazyChange={(value) => updateTypeField({ unit: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

export const RepresentationsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <RepresentationsSectionForm />;
};

const RepresentationsSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, selectRepresentation, deselectRepresentation, hoverRepresentation, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const updateRepresentation = (id: string, representationDiff: any) => {
    applyDiff({
      representations: {
        updated: [{ id, diff: representationDiff }],
      },
    });
  };

  const hasRepresentations = type.representations && type.representations.length > 0;

  return (
    <>
      <TreeItem
        label={t("type.representations")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              applyDiff({
                representations: {
                  added: [{ guid: guid(), url: "", tags: [] }],
                },
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        {hasRepresentations && (
          <SortableTreeItems
            items={(type.representations || []).map((representation: any, index: number) => ({
              ...representation,
              id: `representation-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.representations) return;
              startTransaction();
              applyDiff({
                representations: {
                  removed: type.representations.map((representation: any) => representation.guid),
                  added: arrayMove(type.representations, oldIndex, newIndex),
                },
              });
              finalizeTransaction();
            }}
          >
            {(representation, index) => {
              const isSelected = selection?.representations?.includes(representation.guid) || false;
              const isHovered = hover?.representation === representation.guid;
              return (
                <div onPointerEnter={() => hoverRepresentation(representation.guid)} onPointerLeave={() => clearHover()} onClick={() => (isSelected ? deselectRepresentation(representation.guid) : selectRepresentation(representation.guid))}>
                  <TreeItem
                    key={`representation-${index}`}
                    label={representation.url || `${t("type.representation")} ${index + 1}`}
                    sortable={true}
                    sortableId={`representation-${index}`}
                    isDragHandle={true}
                    className={`${isSelected ? "bg-accent/20" : ""} ${isHovered ? "bg-hover" : ""}`}
                    actions={[
                      {
                        icon: <Minus />,
                        onClick: () => {
                          startTransaction();
                          applyDiff({
                            representations: {
                              removed: [representation.guid],
                            },
                          });
                          finalizeTransaction();
                        },
                        title: t("common.remove"),
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          label={t("type.representationUrl")}
                          value={representation.url}
                          onChange={(e) => {
                            updateRepresentation(representation.guid, { url: e.target.value });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          label={t("type.representationDescription")}
                          value={representation.description || ""}
                          placeholder={t("type.representationDescriptionPlaceholder")}
                          onChange={(e) => {
                            updateRepresentation(representation.guid, { description: e.target.value });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          label={t("type.representationTags")}
                          value={(representation.tags || []).join(", ")}
                          placeholder={t("type.representationTagsPlaceholder")}
                          onChange={(e) => {
                            updateRepresentation(representation.guid, {
                              tags: e.target.value
                                .split(",")
                                .map((tag) => tag.trim())
                                .filter((tag) => tag),
                            });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                  </TreeItem>
                </div>
              );
            }}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const PortsListSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortsListSectionForm />;
};

const PortsListSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction, selectPort, deselectPort, hoverPort, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const updatePort = (id: string, portDiff: any) => {
    const port = type.ports?.find((existingPort) => existingPort.guid === id);
    const diff: any = { ...portDiff };
    if (port) {
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
    }
    applyDiff({
      ports: {
        updated: [{ id, diff }],
      },
    });
  };

  const hasPorts = type.ports && type.ports.length > 0;

  return (
    <>
      <TreeItem
        label={t("type.ports")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              applyDiff({
                ports: {
                  added: [
                    {
                      guid: guid(),
                      t: 0,
                      point: { x: 0, y: 0, z: 0 },
                      direction: { x: 0, y: 0, z: 1 },
                    },
                  ],
                },
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        {hasPorts && (
          <SortableTreeItems
            items={(type.ports || []).map((port: any, index: number) => ({
              ...port,
              id: `port-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.ports) return;
              startTransaction();
              applyDiff({
                ports: {
                  removed: type.ports.map((existingPort: any) => existingPort.guid),
                  added: arrayMove(type.ports, oldIndex, newIndex),
                },
              });
              finalizeTransaction();
            }}
          >
            {(port, index) => {
              const isSelected = selection?.ports?.includes(port.guid) || false;
              const isHovered = hover?.port === port.guid;
              const handleClick = (event: React.MouseEvent) => {
                event.stopPropagation();
                if (isSelected) {
                  deselectPort(port.guid);
                } else {
                  selectPort(port.guid);
                }
              };

              const handleHover = () => {
                hoverPort(port.guid);
              };

              const handleLeave = () => {
                clearHover();
              };

              return (
                <div onPointerEnter={handleHover} onPointerLeave={handleLeave} onClick={handleClick}>
                  <TreeItem
                    key={`port-${index}`}
                    label={port.family || `${t("type.port")} ${index + 1}`}
                    sortable={true}
                    sortableId={`port-${index}`}
                    isDragHandle={true}
                    className={`cursor-pointer ${isSelected ? "ring-1 ring-[color:var(--active-base)]" : ""} ${isHovered ? "bg-[color:var(--hover-base)]" : ""}`}
                    actions={[
                      {
                        icon: <Minus />,
                        onClick: () => {
                          startTransaction();
                          applyDiff({
                            ports: {
                              removed: [port.guid],
                            },
                          });
                          finalizeTransaction();
                        },
                        title: t("common.remove"),
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          label={t("type.portFamily")}
                          value={port.family || ""}
                          placeholder={t("type.portFamilyPlaceholder")}
                          onChange={(e) => {
                            updatePort(port.guid, { family: e.target.value });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          label={t("type.portDescription")}
                          value={port.description || ""}
                          placeholder={t("type.portDescriptionPlaceholder")}
                          onChange={(e) => {
                            updatePort(port.guid, { description: e.target.value });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <div className="flex flex-col gap-1">
                          <label className="text-xs">{t("type.portT")}</label>
                          <Slider
                            value={[port.t ?? 0]}
                            onValueChange={([value]) => {
                              updatePort(port.guid, { t: value });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            min={0}
                            max={1}
                            step={0.01}
                          />
                        </div>
                      </TreeContent>
                    </TreeItem>
                    <TreeItem label={t("type.portPoint")}>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.x")}
                            value={port.point.x}
                            onChange={(value) => {
                              updatePort(port.guid, { point: { x: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.y")}
                            value={port.point.y}
                            onChange={(value) => {
                              updatePort(port.guid, { point: { y: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.z")}
                            value={port.point.z}
                            onChange={(value) => {
                              updatePort(port.guid, { point: { z: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem label={t("type.portDirection")}>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.x")}
                            value={port.direction.x}
                            onChange={(value) => {
                              updatePort(port.guid, { direction: { x: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.y")}
                            value={port.direction.y}
                            onChange={(value) => {
                              updatePort(port.guid, { direction: { y: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            label={t("common.z")}
                            value={port.direction.z}
                            onChange={(value) => {
                              updatePort(port.guid, { direction: { z: value } });
                            }}
                            startTransaction={startTransaction}
                            finalizeTransaction={finalizeTransaction}
                            abortTransaction={abortTransaction}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          label={t("type.portCompatibleFamilies")}
                          value={(port.compatibleFamilies || []).join(", ")}
                          placeholder={t("type.portCompatibleFamiliesPlaceholder")}
                          onChange={(e) => {
                            updatePort(port.guid, {
                              compatibleFamilies: e.target.value
                                .split(",")
                                .map((family) => family.trim())
                                .filter((family) => family),
                            });
                          }}
                          onFocus={startTransaction}
                          onBlur={finalizeTransaction}
                        />
                      </TreeContent>
                    </TreeItem>
                  </TreeItem>
                </div>
              );
            }}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const AuthorsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AuthorsSectionForm />;
};

const AuthorsSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const kit = useKit() as Kit;

  const updateAuthors = (authors: string[]) => {
    kitCommands.updateType(type.guid, { authors });
  };

  const hasAuthors = type.authors && type.authors.length > 0;

  return (
    <>
      <TreeItem
        label={t("type.authors")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              const newAuthorGuid = guid();
              kitCommands.createAuthor({
                guid: newAuthorGuid,
                name: "",
                email: "",
              });
              updateAuthors([...(type.authors || []), newAuthorGuid]);
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        {hasAuthors && (
          <SortableTreeItems
            items={(type.authors || []).map((authorGuid: string, index: number) => {
              const author = kit.authors?.find((a: Author) => a.guid === authorGuid);
              return {
                id: `author-${index}`,
                index,
                guid: authorGuid,
                name: author?.name || "",
                email: author?.email || "",
              };
            })}
            onReorder={(oldIndex, newIndex) => {
              startTransaction();
              updateAuthors(arrayMove(type.authors!, oldIndex, newIndex));
              finalizeTransaction();
            }}
          >
            {(item, index) => (
              <TreeItem
                key={`author-${index}`}
                label={item.name || `${t("type.author")} ${index + 1}`}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      startTransaction();
                      updateAuthors((type.authors || []).filter((_, i: number) => i !== index));
                      finalizeTransaction();
                    },
                    title: t("common.remove"),
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("type.authorName")}
                      value={item.name}
                      onChange={(e) => {
                        kitCommands.updateAuthor(item.guid, { name: e.target.value });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("type.authorEmail")}
                      value={item.email}
                      onChange={(e) => {
                        kitCommands.updateAuthor(item.guid, { email: e.target.value });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const AttributesSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AttributesSectionForm />;
};

const AttributesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const applyDiff = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const updateAttribute = (id: string, attributeDiff: any) => {
    applyDiff({
      attributes: {
        updated: [{ id, diff: attributeDiff }],
      },
    });
  };

  const hasAttributes = type.attributes && type.attributes.length > 0;

  return (
    <>
      <TreeItem
        label={t("type.attributes")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              applyDiff({
                attributes: {
                  added: [{ guid: guid(), key: "" }],
                },
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        {hasAttributes && (
          <SortableTreeItems
            items={(type.attributes || []).map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.attributes) return;
              startTransaction();
              applyDiff({
                attributes: {
                  removed: type.attributes.map((attribute: any) => attribute.guid),
                  added: arrayMove(type.attributes, oldIndex, newIndex),
                },
              });
              finalizeTransaction();
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                label={attribute.key || `${t("type.attribute")} ${index + 1}`}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      startTransaction();
                      applyDiff({
                        attributes: {
                          removed: [attribute.guid],
                        },
                      });
                      finalizeTransaction();
                    },
                    title: t("common.remove"),
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("type.attributeName")}
                      value={attribute.key}
                      onChange={(e) => {
                        updateAttribute(attribute.guid, { key: e.target.value });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("type.attributeValue")}
                      value={attribute.value || ""}
                      placeholder={t("type.attributeValuePlaceholder")}
                      onChange={(e) => {
                        updateAttribute(attribute.guid, { value: e.target.value });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      label={t("type.attributeDefinition")}
                      value={attribute.definition || ""}
                      placeholder={t("type.attributeDefinitionPlaceholder")}
                      onChange={(e) => {
                        updateAttribute(attribute.guid, { definition: e.target.value });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const PortSection: FC<{ portGuid: Guid }> = ({ portGuid }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortSectionForm portGuid={portGuid} />;
};

const PortSectionForm: FC<{ portGuid: Guid }> = ({ portGuid }) => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const port = type.ports?.find((p) => p.guid === portGuid);

  if (!port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("type.portNotFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  const updatePort = (id: string, portDiff: any) => {
    const port = type.ports?.find((existingPort) => existingPort.guid === id);
    const diff: any = { ...portDiff };
    if (port) {
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
    }
    kitCommands.updateType(type.guid, {
      ports: {
        updated: [{ id, diff }],
      },
    });
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            label={t("type.portFamily")}
            value={port.family || ""}
            placeholder={t("type.portFamilyPlaceholder")}
            onChange={(e) => {
              updatePort(port.guid, { family: e.target.value });
            }}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            label={t("type.portDescription")}
            value={port.description || ""}
            placeholder={t("type.portDescriptionPlaceholder")}
            onChange={(e) => {
              updatePort(port.guid, { description: e.target.value });
            }}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("type.portT")}</label>
            <Slider
              value={[port.t ?? 0]}
              onValueChange={([value]) => {
                updatePort(port.guid, { t: value });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("type.portPoint")}>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.x")}
              value={port.point.x}
              onChange={(value) => {
                updatePort(port.guid, { point: { x: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.y")}
              value={port.point.y}
              onChange={(value) => {
                updatePort(port.guid, { point: { y: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.z")}
              value={port.point.z}
              onChange={(value) => {
                updatePort(port.guid, { point: { z: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem label={t("type.portDirection")}>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.x")}
              value={port.direction.x}
              onChange={(value) => {
                updatePort(port.guid, { direction: { x: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.y")}
              value={port.direction.y}
              onChange={(value) => {
                updatePort(port.guid, { direction: { y: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.z")}
              value={port.direction.z}
              onChange={(value) => {
                updatePort(port.guid, { direction: { z: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            label={t("type.portCompatibleFamilies")}
            value={(port.compatibleFamilies || []).join(", ")}
            placeholder={t("type.portCompatibleFamiliesPlaceholder")}
            onChange={(e) => {
              updatePort(port.guid, {
                compatibleFamilies: e.target.value
                  .split(",")
                  .map((family) => family.trim())
                  .filter((family) => family),
              });
            }}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

export const PortsMultipleSection: FC<{ portGuids: Guid[] }> = ({ portGuids }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortsMultipleSectionForm portGuids={portGuids} />;
};

const PortsMultipleSectionForm: FC<{ portGuids: Guid[] }> = ({ portGuids }) => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const ports = type.ports?.filter((p) => portGuids.includes(p.guid)) || [];

  if (ports.length === 0) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{t("type.portsNotFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  const getCommonValue = <T,>(getter: (port: any) => T | undefined): T | undefined => {
    const values = ports.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const updatePorts = (portDiff: any) => {
    startTransaction();
    ports.forEach((port) => {
      const diff: any = { ...portDiff };
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
      kitCommands.updateType(type.guid, {
        ports: {
          updated: [{ id: port.guid, diff }],
        },
      });
    });
    finalizeTransaction();
  };

  const commonFamily = getCommonValue((p) => p.family);
  const commonT = getCommonValue((p) => p.t);
  const commonPointX = getCommonValue((p) => p.point?.x);
  const commonPointY = getCommonValue((p) => p.point?.y);
  const commonPointZ = getCommonValue((p) => p.point?.z);
  const commonDirectionX = getCommonValue((p) => p.direction?.x);
  const commonDirectionY = getCommonValue((p) => p.direction?.y);
  const commonDirectionZ = getCommonValue((p) => p.direction?.z);

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            label={t("type.portFamily")}
            value={commonFamily || ""}
            placeholder={commonFamily === undefined ? t("common.mixedValues") : t("type.portFamilyPlaceholder")}
            onChange={(e) => {
              updatePorts({ family: e.target.value });
            }}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("type.portT")}</label>
            <Slider
              value={[commonT ?? 0]}
              onValueChange={([value]) => {
                updatePorts({ t: value });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem label={t("type.portPoint")}>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.x")}
              value={commonPointX}
              onChange={(value) => {
                updatePorts({ point: { x: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.y")}
              value={commonPointY}
              onChange={(value) => {
                updatePorts({ point: { y: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.z")}
              value={commonPointZ}
              onChange={(value) => {
                updatePorts({ point: { z: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem label={t("type.portDirection")}>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.x")}
              value={commonDirectionX}
              onChange={(value) => {
                updatePorts({ direction: { x: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.y")}
              value={commonDirectionY}
              onChange={(value) => {
                updatePorts({ direction: { y: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              label={t("common.z")}
              value={commonDirectionZ}
              onChange={(value) => {
                updatePorts({ direction: { z: value } });
              }}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
    </>
  );
};
