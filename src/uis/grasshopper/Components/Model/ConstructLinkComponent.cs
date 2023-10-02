using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructLinkComponent : ConstructComponent
    {
        public ConstructLinkComponent() : base("Construct Link", "Link", "", "semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationProtocolParam(), "Protocol", "Pc", "", GH_ParamAccess.item);
            pManager[0].Optional = true;
            pManager.AddTextParameter("Ports", "Po", "", GH_ParamAccess.list);
            pManager[1].Optional = true;
            pManager.AddParameter(new ParameterParam(), "Bias", "B", "", GH_ParamAccess.list);
            pManager[2].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new LinkParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            
            RepresentationProtocol representationProtocol = RepresentationProtocol.None;
            DA.GetData(0, ref representationProtocol);

            var ports = new List<string>();
            DA.GetDataList(1, ports);

            var biasParameters = new List<ParameterGoo>();
            DA.GetDataList(2, biasParameters);

            var connectable = new Link()
            {
                RepresentationProtocol = representationProtocol
            };
            connectable.Ports.AddRange(ports);
            connectable.BiasParameters.AddRange(biasParameters.Select(p=>p.Value));

            DA.SetData(0, new LinkGoo(connectable));
        }
        public override Guid ComponentGuid => new("FC926C0B-59F7-4C85-96E1-484EBD52BD71");
        protected override Bitmap Icon => Resources.icon_construct_link;
    }
}