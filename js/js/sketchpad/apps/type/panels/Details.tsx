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
import { SortableTreeItems, TreeContent, TreeItem } from "../../../../elements/aggregation/Tree";
import { Input } from "../../../../elements/input/Input";
import { Slider } from "../../../../elements/input/Slider";
import Stepper from "../../../../elements/input/Stepper";
import { Textarea } from "../../../../elements/input/Textarea";
import i18n from "../../../../i18n";
import { Author, guid, Guid, Kit, Type } from "../../../../semio";
import { useIsInTypeScope, useKit, useKitCommands, useType } from "../../../kits/store";
import { useTooltip } from "../../../store";
import { useTypeAppCommands, useTypeAppHover, useTypeAppSelection } from "../store";

export const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <TypeDetailsForm />;
};

const TypeDetailsForm: FC = () => {
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const updateTypeField = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.name"
            value={type.name}
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.name", { name: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.name")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.name")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.name")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.description"
            value={type.description || ""}
            placeholderId="semio.sketchpad.app.type.descriptionPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.description", { description: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.description")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.description")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.description")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.icon"
            value={type.icon || ""}
            placeholderId="semio.sketchpad.app.type.iconPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.icon", { icon: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.icon")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.icon")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.icon")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.image"
            value={type.image || ""}
            placeholderId="semio.sketchpad.app.type.imagePlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.image", { image: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.image")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.image")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.image")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.variant"
            value={type.variant || ""}
            placeholderId="semio.sketchpad.app.type.variantPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.variant", { variant: value })}
            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.variant")}
            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.variant")}
            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.variant")}
          />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input
              lazy
              id="semio.sketchpad.app.type.panel.details.section.type.unit"
              value={type.unit}
              onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.unit", { unit: value })}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.type.unit")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.type.unit")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.type.unit")}
            />
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, selectRepresentation, deselectRepresentation, hoverRepresentation, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updateRepresentation = (origin: string, id: string, representationDiff: any) => {
    applyDiff(origin, {
      representations: {
        updated: [{ id, diff: representationDiff }],
      },
    });
  };

  const hasRepresentations = type.representations && type.representations.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.representations"
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.representations.add";
              startTransaction?.(origin);
              applyDiff(origin, {
                representations: {
                  added: [{ guid: guid(), url: "", tags: [] }],
                },
              });
              finalizeTransaction?.(origin);
            },
            id: "semio.sketchpad.common.add",
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
              const origin = "semio.sketchpad.app.type.panel.details.representations.reorder";
              startTransaction?.(origin);
              applyDiff(origin, {
                representations: {
                  removed: type.representations.map((representation: any) => representation.guid),
                  added: arrayMove(type.representations, oldIndex, newIndex),
                },
              });
              finalizeTransaction?.(origin);
            }}
          >
            {(representation, index) => {
              const isSelected = selection?.representations?.includes(representation.guid) || false;
              const isHovered = hover?.representation === representation.guid;
              return (
                <div
                  onPointerEnter={() => hoverRepresentation("semio.sketchpad.app.type.panel.details.representation.hover", representation.guid)}
                  onPointerLeave={() => clearHover("semio.sketchpad.app.type.panel.details.representation.leave")}
                  onClick={() =>
                    isSelected ? deselectRepresentation("semio.sketchpad.app.type.panel.details.representation.deselect", representation.guid) : selectRepresentation("semio.sketchpad.app.type.panel.details.representation.select", representation.guid)
                  }
                >
                  <TreeItem
                    key={`representation-${index}`}
                    id="semio.sketchpad.app.type.representation"
                    label={representation.url}
                    sortable={true}
                    sortableId={`representation-${index}`}
                    isDragHandle={true}
                    className={`${isSelected ? "bg-accent/20" : ""} ${isHovered ? "bg-hover" : ""}`}
                    actions={[
                      {
                        icon: <Minus />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.representations.remove";
                          startTransaction?.(origin);
                          applyDiff(origin, {
                            representations: {
                              removed: [representation.guid],
                            },
                          });
                          finalizeTransaction?.(origin);
                        },
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.representations.url"
                          value={representation.url}
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.url", representation.guid, { url: e.target.value });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.url")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.url")}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          id="semio.sketchpad.app.type.panel.details.section.representations.description"
                          value={representation.description || ""}
                          placeholderId="semio.sketchpad.app.type.representationDescriptionPlaceholder.label"
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.description", representation.guid, { description: e.target.value });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.description")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.description")}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.representations.tags"
                          value={(representation.tags || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.representationTagsPlaceholder.label"
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.tags", representation.guid, {
                              tags: e.target.value
                                .split(",")
                                .map((tag) => tag.trim())
                                .filter((tag) => tag),
                            });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.tags")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.representations.tags")}
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction, selectPort, deselectPort, hoverPort, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updatePort = (origin: string, id: string, portDiff: any) => {
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
    applyDiff(origin, {
      ports: {
        updated: [{ id, diff }],
      },
    });
  };

  const hasPorts = type.ports && type.ports.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.ports"
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.ports.add";
              startTransaction?.(origin);
              applyDiff(origin, {
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
              finalizeTransaction?.(origin);
            },
            id: "semio.sketchpad.common.add",
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
              const origin = "semio.sketchpad.app.type.panel.details.ports.reorder";
              startTransaction?.(origin);
              applyDiff(origin, {
                ports: {
                  removed: type.ports.map((existingPort: any) => existingPort.guid),
                  added: arrayMove(type.ports, oldIndex, newIndex),
                },
              });
              finalizeTransaction?.(origin);
            }}
          >
            {(port, index) => {
              const isSelected = selection?.ports?.includes(port.guid) || false;
              const isHovered = hover?.port === port.guid;
              const handleClick = (event: React.MouseEvent) => {
                event.stopPropagation();
                if (isSelected) {
                  deselectPort("semio.sketchpad.app.type.panel.details.section.ports.deselect", port.guid);
                } else {
                  selectPort("semio.sketchpad.app.type.panel.details.section.ports.select", port.guid);
                }
              };

              const handleHover = () => {
                hoverPort("semio.sketchpad.app.type.panel.details.section.ports.hover", port.guid);
              };

              const handleLeave = () => {
                clearHover("semio.sketchpad.app.type.panel.details.section.ports.leave");
              };

              return (
                <div onPointerEnter={handleHover} onPointerLeave={handleLeave} onClick={handleClick}>
                  <TreeItem
                    key={`port-${index}`}
                    id="semio.sketchpad.app.type.port"
                    label={port.family}
                    sortable={true}
                    sortableId={`port-${index}`}
                    isDragHandle={true}
                    className={`cursor-selectable ${isSelected ? "ring-1 ring-[color:var(--active-base)]" : ""} ${isHovered ? "bg-[color:var(--hover-base)]" : ""}`}
                    actions={[
                      {
                        icon: <Minus />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.ports.remove";
                          startTransaction?.(origin);
                          applyDiff(origin, {
                            ports: {
                              removed: [port.guid],
                            },
                          });
                          finalizeTransaction?.(origin);
                        },
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.ports.family"
                          value={port.family || ""}
                          placeholderId="semio.sketchpad.app.type.portFamilyPlaceholder.label"
                          onChange={(e) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.family", port.guid, { family: e.target.value });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          id="semio.sketchpad.app.type.panel.details.section.ports.description"
                          value={port.description || ""}
                          placeholderId="semio.sketchpad.app.type.portDescriptionPlaceholder.label"
                          onChange={(e) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.description", port.guid, { description: e.target.value });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.description")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.description")}
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <div className="flex flex-col gap-1">
                          <label className="text-xs">{i18n.t("semio.sketchpad.app.type.portT")}</label>
                          <Slider
                            value={[port.t ?? 0]}
                            onValueChange={([value]) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.t", port.guid, { t: value });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
                            min={0}
                            max={1}
                            step={0.01}
                          />
                        </div>
                      </TreeContent>
                    </TreeItem>
                    <TreeItem id="semio.sketchpad.app.type.portPoint">
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
                            value={port.point.x}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
                            value={port.point.y}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
                            value={port.point.z}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem id="semio.sketchpad.app.type.portDirection">
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
                            value={port.direction.x}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
                            value={port.direction.y}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
                            value={port.direction.z}
                            onChange={(value) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
                            }}
                            startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
                            finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
                            abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies"
                          value={(port.compatibleFamilies || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.portCompatibleFamiliesPlaceholder.label"
                          onChange={(e) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies", port.guid, {
                              compatibleFamilies: e.target.value
                                .split(",")
                                .map((family) => family.trim())
                                .filter((family) => family),
                            });
                          }}
                          onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies")}
                          onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies")}
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const kit = useKit() as Kit;

  const updateAuthors = (origin: string, authors: string[]) => {
    kitCommands?.updateType(origin, type.guid, { authors });
  };

  const hasAuthors = type.authors && type.authors.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.authors"
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.authors.add";
              startTransaction?.(origin);
              const newAuthorGuid = guid();
              kitCommands?.createAuthor(origin, {
                guid: newAuthorGuid,
                name: "",
                email: "",
              });
              updateAuthors(origin, [...(type.authors || []), newAuthorGuid]);
              finalizeTransaction?.(origin);
            },
            id: "semio.sketchpad.common.add",
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
              const origin = "semio.sketchpad.app.type.panel.details.authors.reorder";
              startTransaction?.(origin);
              updateAuthors(origin, arrayMove(type.authors!, oldIndex, newIndex));
              finalizeTransaction?.(origin);
            }}
          >
            {(item, index) => (
              <TreeItem
                key={`author-${index}`}
                id="semio.sketchpad.app.type.author"
                label={item.name}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.type.panel.details.authors.remove";
                      startTransaction?.(origin);
                      updateAuthors(
                        origin,
                        (type.authors || []).filter((_, i: number) => i !== index),
                      );
                      finalizeTransaction?.(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.authors.name"
                      value={item.name}
                      onChange={(e) => {
                        kitCommands?.updateAuthor("semio.sketchpad.app.type.panel.details.section.authors.name", item.guid, { name: e.target.value });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.authors.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.authors.name")}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.authors.email"
                      value={item.email}
                      onChange={(e) => {
                        kitCommands?.updateAuthor("semio.sketchpad.app.type.panel.details.section.authors.email", item.guid, { email: e.target.value });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.authors.email")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.authors.email")}
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updateAttribute = (origin: string, id: string, attributeDiff: any) => {
    applyDiff(origin, {
      attributes: {
        updated: [{ id, diff: attributeDiff }],
      },
    });
  };

  const hasAttributes = type.attributes && type.attributes.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.attributes"
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.attributes.add";
              startTransaction?.(origin);
              applyDiff(origin, {
                attributes: {
                  added: [{ guid: guid(), key: "" }],
                },
              });
              finalizeTransaction?.(origin);
            },
            id: "semio.sketchpad.common.add",
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
              const origin = "semio.sketchpad.app.type.panel.details.attributes.reorder";
              startTransaction?.(origin);
              applyDiff(origin, {
                attributes: {
                  removed: type.attributes.map((attribute: any) => attribute.guid),
                  added: arrayMove(type.attributes, oldIndex, newIndex),
                },
              });
              finalizeTransaction?.(origin);
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                id="semio.sketchpad.app.type.attribute"
                label={attribute.key}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.type.panel.details.attributes.remove";
                      startTransaction?.(origin);
                      applyDiff(origin, {
                        attributes: {
                          removed: [attribute.guid],
                        },
                      });
                      finalizeTransaction?.(origin);
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.name"
                      value={attribute.key}
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.name", attribute.guid, { key: e.target.value });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.name")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.name")}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.value"
                      value={attribute.value || ""}
                      placeholderId="semio.sketchpad.app.type.attributeValuePlaceholder.label"
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.value", attribute.guid, { value: e.target.value });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.value")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.value")}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.definition"
                      value={attribute.definition || ""}
                      placeholderId="semio.sketchpad.app.type.attributeDefinitionPlaceholder.label"
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.definition", attribute.guid, { definition: e.target.value });
                      }}
                      onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.definition")}
                      onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.attributes.definition")}
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const port = type.ports?.find((p) => p.guid === portGuid);

  if (!port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{i18n.t("semio.sketchpad.app.type.portNotFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  const updatePort = (origin: string, id: string, portDiff: any) => {
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
    kitCommands?.updateType(origin, type.guid, {
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
            id="semio.sketchpad.app.type.panel.details.section.ports.family"
            value={port.family || ""}
            placeholderId="semio.sketchpad.app.type.portFamilyPlaceholder.label"
            onChange={(e) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.family", port.guid, { family: e.target.value });
            }}
            onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
            onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            id="semio.sketchpad.app.type.panel.details.section.ports.description"
            value={port.description || ""}
            placeholderId="semio.sketchpad.app.type.portDescriptionPlaceholder.label"
            onChange={(e) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.description", port.guid, { description: e.target.value });
            }}
            onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.description")}
            onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.description")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{i18n.t("semio.sketchpad.app.type.portT")}</label>
            <Slider
              value={[port.t ?? 0]}
              onValueChange={([value]) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.t", port.guid, { t: value });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portPoint">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
              value={port.point.x}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
              value={port.point.y}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
              value={port.point.z}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portDirection">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
              value={port.direction.x}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
              value={port.direction.y}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
              value={port.direction.z}
              onChange={(value) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            id="semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies"
            value={(port.compatibleFamilies || []).join(", ")}
            placeholderId="semio.sketchpad.app.type.portCompatibleFamiliesPlaceholder.label"
            onChange={(e) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies", port.guid, {
                compatibleFamilies: e.target.value
                  .split(",")
                  .map((family) => family.trim())
                  .filter((family) => family),
              });
            }}
            onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies")}
            onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies")}
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
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const ports = type.ports?.filter((p) => portGuids.includes(p.guid)) || [];

  if (ports.length === 0) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{i18n.t("semio.sketchpad.app.type.portsNotFound")}</p>
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

  const updatePorts = (origin: string, portDiff: any) => {
    startTransaction?.(origin);
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
      kitCommands?.updateType(origin, type.guid, {
        ports: {
          updated: [{ id: port.guid, diff }],
        },
      });
    });
    finalizeTransaction?.(origin);
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
            id="semio.sketchpad.app.type.panel.details.section.ports.family"
            value={commonFamily || ""}
            placeholderId={commonFamily === undefined ? "semio.sketchpad.common.mixedValues" : "semio.sketchpad.app.type.portFamilyPlaceholder.label"}
            onChange={(e) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.ports.family", { family: e.target.value });
            }}
            onFocus={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
            onBlur={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.family")}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{i18n.t("semio.sketchpad.app.type.portT")}</label>
            <Slider
              value={[commonT ?? 0]}
              onValueChange={([value]) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.t", { t: value });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t")}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portPoint">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
              value={commonPointX}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.x", { point: { x: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
              value={commonPointY}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.y", { point: { y: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
              value={commonPointZ}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.z", { point: { z: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portDirection">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
              value={commonDirectionX}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.x", { direction: { x: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
              value={commonDirectionY}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.y", { direction: { y: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
              value={commonDirectionZ}
              onChange={(value) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.z", { direction: { z: value } });
              }}
              startTransaction={() => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              finalizeTransaction={() => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              abortTransaction={() => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
    </>
  );
};
