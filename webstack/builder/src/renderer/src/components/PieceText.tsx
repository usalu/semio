import { FC } from 'react';
import { type INode} from 'react-digraph'

type IPieceTextProps = {
  id: string,
  isSelected: boolean,
};


const PieceText: (FC <IPieceTextProps>) = ({id, isSelected})  => {
  return (
    <text className="node-text" style={
      {
        textAnchor: "middle",
        transform: "translateY(5px)"
      }
    }>{id}</text>
  );
}

export default PieceText;
