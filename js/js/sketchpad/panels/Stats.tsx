// #region Header

// Stats.tsx

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

import { FC } from "react";
import Panel from "../Panel";
import { ResizablePanelProps } from "../Sketchpad";

interface StatsProps extends ResizablePanelProps {}

const Stats: FC<StatsProps> = ({ visible, onWidthChange, width }) => {
  return <Panel panelId="stats" visible={visible} onWidthChange={onWidthChange} width={width} resizeSide="right" zIndex={30} showBackground={false} emptyMessage="No stats sections available" />;
};

export default Stats;
