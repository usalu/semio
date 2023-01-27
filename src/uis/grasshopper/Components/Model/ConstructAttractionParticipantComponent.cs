using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Google.Protobuf.Collections;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Assembler.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructConnectableComponent : ConstructComponent
    {
        public ConstructConnectableComponent() : base("Construct Connection Participant", "Connection Participant", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            //pManager.AddTextParameter(field.Name, "PI", "", GH_ParamAccess.item);
            pManager.AddParameter(new SobjectParam(),"Participant", "P", "", GH_ParamAccess.item);
            pManager.AddParameter(new RepresentationProtocolParam(), "Protocol", "Pc", "", GH_ParamAccess.item);
            pManager[1].Optional = true;
            pManager.AddTextParameter("Ports", "Po", "", GH_ParamAccess.list);
            pManager[2].Optional = true;
            pManager.AddParameter(new ParameterParam(), "Bias", "B", "", GH_ParamAccess.list);
            pManager[3].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ConnectableParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //string connectableId = "";
            //if (!DA.GetData(0, ref connectableId)) return;

            SobjectGoo connectable = new();
            if (!DA.GetData(0, ref connectable)) return;

            RepresentationProtocol representationProtocol = RepresentationProtocol.None;
            DA.GetData(1, ref representationProtocol);

            var ports = new List<string>();
            DA.GetDataList(2, ports);

            var bias = new List<ParameterGoo>();
            DA.GetDataList(3, bias);

            var connectable = new Connectable()
            {
                ParticipantId = connectable.Value.Id,
                RepresentationProtocol = representationProtocol,
            };
            connectable.Ports.AddRange(ports);
            connectable.Bias.Add(new MapField<string, string>
            {
                bias.Select(x => new { x.Value.Name, x.Value.Value }).ToDictionary(x => x.Name, x => x.Value)
            });

            DA.SetData(0, new ConnectableGoo(connectable));
        }
        public override Guid ComponentGuid => new("C20F4D78-1178-4700-973F-6AB81DAC35F1");
        protected override Bitmap Icon => Resources.icon_construct_connectionconnectable;
    }
}