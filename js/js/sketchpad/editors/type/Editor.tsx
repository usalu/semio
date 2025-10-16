// #region Header

// TypeEditor.tsx

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
import { FC, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { SortableTreeItems, TreeContent, TreeItem } from "../../../elements/aggregation/Tree";
import { Input } from "../../../elements/input/Input";
import Stepper from "../../../elements/input/Stepper";
import { Textarea } from "../../../elements/input/Textarea";
import { guid, Type } from "../../../semio";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, useEditorType, useIsInTypeScope, useKitCommands, useType, useTypeEditorCommands } from "../../store";
import TypeScene from "./canvas/Scene";

const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();

  if (!isInTypeScope) {
    return null;
  }

  return <TypeDetailsForm />;
};

const TypeDetailsForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

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

const RepresentationsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <RepresentationsSectionForm />;
};

const RepresentationsSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const handleChange = (updatedType: any) => {
    kitCommands.updateType(type.guid, updatedType);
  };

  return (
    <>
      <TreeItem
        label={t("type.representations")}
        actions={[
          {
            icon: <Plus size={12} />,
            onClick: () => {
              startTransaction();
              handleChange({
                ...type,
                representations: [...(type.representations || []), { guid: guid(), url: "", tags: [] }],
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        <SortableTreeItems
          items={(type.representations || []).map((representation: any, index: number) => ({
            ...representation,
            id: `representation-${index}`,
            index,
          }))}
          onReorder={(oldIndex, newIndex) => {
            startTransaction();
            handleChange({
              ...type,
              representations: arrayMove(type.representations!, oldIndex, newIndex),
            });
            finalizeTransaction();
          }}
        >
          {(representation, index) => (
            <TreeItem
              key={`representation-${index}`}
              label={representation.url || `${t("type.representation")} ${index + 1}`}
              sortable={true}
              sortableId={`representation-${index}`}
              isDragHandle={true}
              actions={[
                {
                  icon: <Minus size={12} />,
                  onClick: () => {
                    startTransaction();
                    handleChange({
                      ...type,
                      representations: type.representations?.filter((_: any, i: number) => i !== index),
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
                      const updatedRepresentations = [...(type.representations || [])];
                      updatedRepresentations[index] = {
                        ...representation,
                        url: e.target.value,
                      };
                      handleChange({ ...type, representations: updatedRepresentations });
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
                      const updatedRepresentations = [...(type.representations || [])];
                      updatedRepresentations[index] = {
                        ...representation,
                        description: e.target.value,
                      };
                      handleChange({ ...type, representations: updatedRepresentations });
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
                      const updatedRepresentations = [...(type.representations || [])];
                      updatedRepresentations[index] = {
                        ...representation,
                        tags: e.target.value
                          .split(",")
                          .map((tag) => tag.trim())
                          .filter((tag) => tag),
                      };
                      handleChange({ ...type, representations: updatedRepresentations });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeContent>
              </TreeItem>
            </TreeItem>
          )}
        </SortableTreeItems>
      </TreeItem>
    </>
  );
};

const PortsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortsSectionForm />;
};

const PortsSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const handleChange = (updatedType: any) => {
    kitCommands.updateType(type.guid, updatedType);
  };

  return (
    <>
      <TreeItem
        label={t("type.ports")}
        actions={[
          {
            icon: <Plus size={12} />,
            onClick: () => {
              startTransaction();
              handleChange({
                ...type,
                ports: [
                  ...(type.ports || []),
                  {
                    guid: guid(),
                    t: 0,
                    point: { x: 0, y: 0, z: 0 },
                    direction: { x: 0, y: 0, z: 1 },
                  },
                ],
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        <SortableTreeItems
          items={(type.ports || []).map((port: any, index: number) => ({
            ...port,
            id: `port-${index}`,
            index,
          }))}
          onReorder={(oldIndex, newIndex) => {
            startTransaction();
            handleChange({
              ...type,
              ports: arrayMove(type.ports!, oldIndex, newIndex),
            });
            finalizeTransaction();
          }}
        >
          {(port, index) => (
            <TreeItem
              key={`port-${index}`}
              label={port.family || `${t("type.port")} ${index + 1}`}
              sortable={true}
              sortableId={`port-${index}`}
              isDragHandle={true}
              actions={[
                {
                  icon: <Minus size={12} />,
                  onClick: () => {
                    startTransaction();
                    handleChange({
                      ...type,
                      ports: type.ports?.filter((_: any, i: number) => i !== index),
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
                      const updatedPorts = [...(type.ports || [])];
                      updatedPorts[index] = {
                        ...port,
                        family: e.target.value,
                      };
                      handleChange({ ...type, ports: updatedPorts });
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
                      const updatedPorts = [...(type.ports || [])];
                      updatedPorts[index] = {
                        ...port,
                        description: e.target.value,
                      };
                      handleChange({ ...type, ports: updatedPorts });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeContent>
              </TreeItem>
              <TreeItem label={t("type.portPoint")}>
                <TreeItem>
                  <TreeContent>
                    <Stepper
                      label={t("common.x")}
                      value={port.point.x}
                      onChange={(value) => {
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          point: { ...port.point, x: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          point: { ...port.point, y: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          point: { ...port.point, z: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          direction: { ...port.direction, x: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          direction: { ...port.direction, y: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                        const updatedPorts = [...(type.ports || [])];
                        updatedPorts[index] = {
                          ...port,
                          direction: { ...port.direction, z: value },
                        };
                        handleChange({ ...type, ports: updatedPorts });
                      }}
                      onPointerDown={startTransaction}
                      onPointerUp={finalizeTransaction}
                      onPointerCancel={abortTransaction}
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
                      const updatedPorts = [...(type.ports || [])];
                      updatedPorts[index] = {
                        ...port,
                        compatibleFamilies: e.target.value
                          .split(",")
                          .map((family) => family.trim())
                          .filter((family) => family),
                      };
                      handleChange({ ...type, ports: updatedPorts });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeContent>
              </TreeItem>
            </TreeItem>
          )}
        </SortableTreeItems>
      </TreeItem>
    </>
  );
};

const AuthorsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AuthorsSectionForm />;
};

const AuthorsSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const handleChange = (updatedType: any) => {
    kitCommands.updateType(type.guid, updatedType);
  };

  return (
    <>
      <TreeItem
        label={t("type.authors")}
        actions={[
          {
            icon: <Plus size={12} />,
            onClick: () => {
              startTransaction();
              handleChange({
                ...type,
                authors: [...(type.authors || []), ""],
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        <SortableTreeItems
          items={(type.authors || []).map((author: string, index: number) => ({
            id: `author-${index}`,
            index,
            name: author,
          }))}
          onReorder={(oldIndex, newIndex) => {
            startTransaction();
            handleChange({
              ...type,
              authors: arrayMove(type.authors!, oldIndex, newIndex),
            });
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
                  icon: <Minus size={12} />,
                  onClick: () => {
                    startTransaction();
                    handleChange({
                      ...type,
                      authors: type.authors?.filter((_: any, i: number) => i !== index),
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
                    label={t("type.authorName")}
                    value={item.name}
                    onChange={(e) => {
                      const updatedAuthors = [...(type.authors || [])];
                      updatedAuthors[index] = e.target.value;
                      handleChange({ ...type, authors: updatedAuthors });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeContent>
              </TreeItem>
            </TreeItem>
          )}
        </SortableTreeItems>
      </TreeItem>
    </>
  );
};

const AttributesSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AttributesSectionForm />;
};

const AttributesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction } = useTypeEditorCommands();
  const kitCommands = useKitCommands();
  const type = useType() as Type;

  const updateTypeField = (diff: any) => {
    kitCommands.updateType(type.guid, diff);
  };

  const handleChange = (updatedType: any) => {
    kitCommands.updateType(type.guid, updatedType);
  };

  return (
    <>
      <TreeItem
        label={t("type.attributes")}
        actions={[
          {
            icon: <Plus size={12} />,
            onClick: () => {
              startTransaction();
              handleChange({
                ...type,
                attributes: [...(type.attributes || []), { guid: guid(), key: "" }],
              });
              finalizeTransaction();
            },
            title: t("common.add"),
          },
        ]}
      >
        <SortableTreeItems
          items={(type.attributes || []).map((attribute: any, index: number) => ({
            ...attribute,
            id: `attribute-${index}`,
            index,
          }))}
          onReorder={(oldIndex, newIndex) => {
            startTransaction();
            handleChange({
              ...type,
              attributes: arrayMove(type.attributes!, oldIndex, newIndex),
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
                  icon: <Minus size={12} />,
                  onClick: () => {
                    startTransaction();
                    handleChange({
                      ...type,
                      attributes: type.attributes?.filter((_: any, i: number) => i !== index),
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
                      const updatedAttributes = [...(type.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        key: e.target.value,
                      };
                      handleChange({ ...type, attributes: updatedAttributes });
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
                      const updatedAttributes = [...(type.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        value: e.target.value,
                      };
                      handleChange({ ...type, attributes: updatedAttributes });
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
                      const updatedAttributes = [...(type.attributes || [])];
                      updatedAttributes[index] = {
                        ...attribute,
                        definition: e.target.value,
                      };
                      handleChange({ ...type, attributes: updatedAttributes });
                    }}
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeContent>
              </TreeItem>
            </TreeItem>
          )}
        </SortableTreeItems>
      </TreeItem>
    </>
  );
};

const Editor: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

  useEffect(() => {
    if (editorType !== EditorType.TYPE) return;

    addSection("details", {
      id: "type-details",
      label: "Type",
      order: 0,
      defaultOpen: true,
      content: () => <TypeDetails />,
    });

    addSection("details", {
      id: "type-representations",
      label: "Representations",
      order: 1,
      defaultOpen: true,
      content: () => <RepresentationsSection />,
    });

    addSection("details", {
      id: "type-ports",
      label: "Ports",
      order: 2,
      defaultOpen: true,
      content: () => <PortsSection />,
    });

    addSection("details", {
      id: "type-authors",
      label: "Authors",
      order: 3,
      defaultOpen: true,
      content: () => <AuthorsSection />,
    });

    addSection("details", {
      id: "type-attributes",
      label: "Attributes",
      order: 4,
      defaultOpen: true,
      content: () => <AttributesSection />,
    });

    return () => {
      removeSection("details", "type-details");
      removeSection("details", "type-representations");
      removeSection("details", "type-ports");
      removeSection("details", "type-authors");
      removeSection("details", "type-attributes");
    };
  }, [addSection, removeSection, editorType]);

  return <TypeScene />;
};

export default Editor;
