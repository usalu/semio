using System;
using System.Drawing;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructConnectionComponent : ConstructComponent
    {
        public ConstructConnectionComponent() : base("Construct Connection", "Connection", "", "semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new ConnectableParam(),"Connecting", "Cn", "", GH_ParamAccess.item);
            pManager.AddParameter(new ConnectableParam(), "Connected", "Cd", "", GH_ParamAccess.item);
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ConnectionParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            ConnectableGoo connecting = new();
            if (!DA.GetData(0, ref connecting)) return;

            ConnectableGoo connected = new();
            if (!DA.GetData(1, ref connected)) return;

            DA.SetData(0, new ConnectionGoo(new Connection()
            {
                Connecting = connecting.Value,
                Connected = connected.Value,
            }));
        }
        public override Guid ComponentGuid => new("2761156E-6A70-4CAC-B50B-42F410C406D2");
        protected override Bitmap Icon => Resources.icon_construct_connection;
    }
}