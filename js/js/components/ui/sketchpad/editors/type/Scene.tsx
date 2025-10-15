// #region Header

// TypeModel.tsx

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

import { FC, useCallback } from "react";
import { useTypeEditorCamera, useTypeEditorCommands } from "../../../../../store";
import { Camera } from "../../../../../semio";
import SceneComponent from "../../../elements/Scene";

const TypeModel: FC = () => {
  const { setCamera } = useTypeEditorCommands();
  const camera = useTypeEditorCamera();
  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      setCamera(newCamera);
    },
    [setCamera],
  );

  return <SceneComponent camera={camera} onCameraChange={onCameraChange} />;
};

export default TypeModel;
